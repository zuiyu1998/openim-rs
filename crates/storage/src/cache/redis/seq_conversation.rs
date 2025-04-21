use std::{sync::Arc, time::Duration};

use crate::{cache::SeqConversationCache, database::seq_conversation::SeqConversationRepo};

use abi::{
    async_trait::async_trait, protocol::pb::msg_processor, redis, utils::time_util, ErrorKind,
    Result,
};

use super::consts::get_malloc_seq_key;

pub struct SeqConversationRedis {
    client: redis::Client,
    lock_time: Duration,
    data_time: Duration,
    seq_conversation_database: Arc<dyn SeqConversationRepo>,
}

fn get_seq_malloc_key(conversation_id: &str) -> String {
    get_malloc_seq_key(conversation_id)
}

pub enum MallocResult {
    Unknown(i64),
    Success {
        curr_seq: i64,
        last_seq: i64,
        malloc_time: i64,
    },
    NotFound {
        lock_value: i64,
        malloc_time: i64,
    },
    Locked,
    Exceeded {
        curr_seq: i64,
        last_seq: i64,
        lock_value: i64,
        malloc_time: i64,
    },
}

impl Default for MallocResult {
    fn default() -> Self {
        MallocResult::Unknown(4)
    }
}

impl From<Vec<i64>> for MallocResult {
    fn from(value: Vec<i64>) -> Self {
        match value[0] {
            0 => MallocResult::Success {
                curr_seq: value[1],
                last_seq: value[2],
                malloc_time: value[3],
            },
            1 => MallocResult::NotFound {
                lock_value: value[1],
                malloc_time: value[2],
            },
            2 => MallocResult::Locked,
            3 => MallocResult::Exceeded {
                curr_seq: value[1],
                last_seq: value[2],
                lock_value: value[3],
                malloc_time: value[4],
            },
            _ => MallocResult::Unknown(value[0]),
        }
    }
}

impl SeqConversationRedis {
    pub fn new(
        client: redis::Client,
        seq_conversation_database: Arc<dyn SeqConversationRepo>,
    ) -> SeqConversationRedis {
        SeqConversationRedis {
            client,
            lock_time: Duration::from_secs(3),
            data_time: Duration::from_secs(60 * 60 * 24 * 365),
            seq_conversation_database,
        }
    }

    async fn set_seq(
        &self,
        key: &str,
        owner: i64,
        curr_seq: i64,
        last_seq: i64,
        mill: i64,
    ) -> Result<i64> {
        if last_seq < curr_seq {
            return Err(ErrorKind::SizeIsSmall.into());
        }
        let script = r#"
        local key = KEYS[1]
        local lockValue = ARGV[1]
        local dataSecond = ARGV[2]
        local currSeq = tonumber(ARGV[3])
        local lastSeq = tonumber(ARGV[4])
        local mallocTime = ARGV[5]
        if redis.call("EXISTS", key) == 0 then
            redis.call("HSET", key, "CURR", currSeq, "LAST", lastSeq, "TIME", mallocTime)
            redis.call("EXPIRE", key, dataSecond)
            return 1
        end
        if redis.call("HGET", key, "LOCK") ~= lockValue then
            return 2
        end
        redis.call("HDEL", key, "LOCK")
        redis.call("HSET", key, "CURR", currSeq, "LAST", lastSeq, "TIME", mallocTime)
        redis.call("EXPIRE", key, dataSecond)
        return 0
        "#;

        let mut conn = self.client.get_multiplexed_async_connection().await?;

        let result: i64 = redis::Script::new(script)
            .key(key)
            .arg(owner)
            .arg(self.data_time.as_secs())
            .arg(curr_seq)
            .arg(last_seq)
            .arg(mill)
            .invoke_async(&mut conn)
            .await?;

        Ok(result)
    }

    async fn set_seq_retry(&self, key: &str, owner: i64, curr_seq: i64, last_seq: i64, mill: i64) {
        for _i in 0..10 {
            let state = match self.set_seq(key, owner, curr_seq, last_seq, mill).await {
                Err(e) => {
                    tracing::error!(
                        "set seq cache failed. Error is {}. args: {} {} {} {} {}",
                        e,
                        key,
                        owner,
                        curr_seq,
                        last_seq,
                        mill
                    );
                    continue;
                }
                Ok(state) => state,
            };

            match state {
                0 | 1 => {
                    tracing::warn!(
                        "set seq cache lock not found. args: {} {} {} {} {}",
                        key,
                        owner,
                        curr_seq,
                        last_seq,
                        mill
                    );
                }
                2 => {
                    tracing::warn!(
                        "set seq cache lock not found. args: {} {} {} {} {}",
                        key,
                        owner,
                        curr_seq,
                        last_seq,
                        mill
                    );
                }
                _ => {
                    tracing::error!(
                        "set seq cache lock to be held by someone else. args: {} {} {} {} {}",
                        key,
                        owner,
                        curr_seq,
                        last_seq,
                        mill
                    );
                }
            }

            tracing::error!(
                "set seq cache retrying still failed. args: {} {} {} {} {}",
                key,
                owner,
                curr_seq,
                last_seq,
                mill
            );
            return;
        }
    }

    fn get_malloc_size(&self, conversation_id: &str, size: i64) -> i64 {
        if size == 0 {
            return 0;
        }

        let mut basic_size;

        if msg_processor::is_group_conversation_id(conversation_id) {
            basic_size = 100;
        } else {
            basic_size = 50;
        }

        basic_size += size;

        basic_size
    }

    async fn malloc_time(&self, conversation_id: &str, size: i64) -> Result<(i64, i64)> {
        if size < 0 {
            return Err(ErrorKind::SizeIsNegative.into());
        }

        let key = get_seq_malloc_key(conversation_id);

        for _ in 0..10 {
            let res = self.malloc_size(&key, size).await?;

            match res {
                MallocResult::Success {
                    curr_seq,
                    last_seq: _,
                    malloc_time,
                } => {
                    return Ok((curr_seq, malloc_time));
                }
                MallocResult::NotFound {
                    lock_value,
                    malloc_time,
                } => {
                    let malloc_size = self.get_malloc_size(conversation_id, size);
                    let seq = self
                        .seq_conversation_database
                        .malloc(conversation_id, size)
                        .await?;

                    self.set_seq_retry(
                        &key,
                        lock_value,
                        seq + size,
                        seq + malloc_size,
                        malloc_time,
                    )
                    .await;

                    return Ok((seq, malloc_time));
                }
                MallocResult::Locked => {
                    continue;
                }
                // exceeded cache max value
                MallocResult::Exceeded {
                    curr_seq,
                    last_seq,
                    lock_value,
                    malloc_time,
                } => {
                    let malloc_size = self.get_malloc_size(conversation_id, size);
                    let seq = self
                        .seq_conversation_database
                        .malloc(conversation_id, malloc_size)
                        .await?;

                    if last_seq == seq {
                        self.set_seq_retry(
                            &key,
                            lock_value,
                            curr_seq + size,
                            seq + malloc_size,
                            malloc_time,
                        )
                        .await;
                    } else {
                        tracing::warn!(
                            "malloc seq not equal cache last seq. args is: conversation_id: {}, curr_seq: {}, last_seq: {}, seq: {}",
                            conversation_id, curr_seq,last_seq,seq
                        );
                        return Ok((seq, malloc_time));
                    }
                }
                MallocResult::Unknown(state) => {
                    tracing::error!(
                        "malloc seq unknown state. args is: state: {}, conversation_id: {}, size: {}",
                        state,
                        conversation_id,
                        size
                    );
                    return Err(ErrorKind::MallocUnknownState(state).into());
                }
            }
        }

        tracing::error!(
            "malloc seq retrying still failed: conversation_id: {}, size: {}",
            conversation_id,
            size
        );

        Err(ErrorKind::MallocLockTimeout.into())
    }

    async fn malloc_size(&self, key: &str, size: i64) -> Result<MallocResult> {
        let script = r#"
            local key = KEYS[1]
            local size = tonumber(ARGV[1])
            local lockSecond = ARGV[2]
            local dataSecond = ARGV[3]
            local mallocTime = ARGV[4]
            local result = {}
            if redis.call("EXISTS", key) == 0 then
                local lockValue = math.random(0, 999999999)
                redis.call("HSET", key, "LOCK", lockValue)
                redis.call("EXPIRE", key, lockSecond)
                table.insert(result, 1)
                table.insert(result, lockValue)
                table.insert(result, mallocTime)
                return result
            end
            if redis.call("HEXISTS", key, "LOCK") == 1 then
                table.insert(result, 2)
                return result
            end
            local currSeq = tonumber(redis.call("HGET", key, "CURR"))
            local lastSeq = tonumber(redis.call("HGET", key, "LAST"))
            if size == 0 then
                redis.call("EXPIRE", key, dataSecond)
                table.insert(result, 0)
                table.insert(result, currSeq)
                table.insert(result, lastSeq)
                local setTime = redis.call("HGET", key, "TIME")
                if setTime then
                    table.insert(result, setTime)	
                else
                    table.insert(result, 0)
                end
                return result
            end
            local maxSeq = currSeq + size
            if maxSeq > lastSeq then
                local lockValue = math.random(0, 999999999)
                redis.call("HSET", key, "LOCK", lockValue)
                redis.call("HSET", key, "CURR", lastSeq)
                redis.call("HSET", key, "TIME", mallocTime)
                redis.call("EXPIRE", key, lockSecond)
                table.insert(result, 3)
                table.insert(result, currSeq)
                table.insert(result, lastSeq)
                table.insert(result, lockValue)
                table.insert(result, mallocTime)
                return result
            end
            redis.call("HSET", key, "CURR", max_seq)
            redis.call("HSET", key, "TIME", ARGV[4])
            redis.call("EXPIRE", key, dataSecond)
            table.insert(result, 0)
            table.insert(result, curr_seq)
            table.insert(result, last_seq)
            table.insert(result, mallocTime)
            return result
        "#;

        let mut conn = self.client.get_multiplexed_async_connection().await?;

        let malloc_time = time_util::now().to_utc().timestamp_millis();

        let result: Vec<i64> = redis::Script::new(script)
            .key(key)
            .arg(size)
            .arg(self.lock_time.as_secs())
            .arg(self.data_time.as_secs())
            .arg(malloc_time)
            .invoke_async(&mut conn)
            .await?;
        Ok(result.into())
    }
}

#[async_trait]
impl SeqConversationCache for SeqConversationRedis {
    async fn malloc(&self, conversation_id: &str, size: i64) -> Result<i64> {
        let (seq, _) = self.malloc_time(conversation_id, size).await?;
        Ok(seq)
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;

    use abi::tokio;

    use crate::database::mongodb::{
        new_mongo_database, seq_conversation::SeqConversationMongodb, MongodbConfig,
    };

    #[tokio::test]
    async fn test_seq_user_redis() {
        use super::SeqConversationRedis;
        use crate::cache::redis::{
            new_redis_client, seq_conversation::SeqConversationCache, RedisConfig,
        };

        let redis_config = RedisConfig {
            host: "192.168.0.230".to_string(),
            port: 6379,
        };

        let config = MongodbConfig {
            host: "127.0.0.1".to_string(),
            port: 27017,
            user: "openIM".to_string(),
            password: "openIM123".to_string(),
            database: "test".to_string(),
            seq_user_name: "seq_user".to_string(),
            seq_conversation_name: "seq_conversation_name".to_string(),
        };

        let client = new_redis_client(&redis_config).unwrap();

        let database = new_mongo_database(&config).await.unwrap();

        let seq_conversation =
            SeqConversationMongodb::new(&database, &config.seq_conversation_name)
                .await
                .unwrap();
        let conversation_id = "test_conversation".to_string();
        let seq = 10;

        let seq_conversation_redis = SeqConversationRedis::new(client, Arc::new(seq_conversation));

        let malloc_seq = seq_conversation_redis
            .malloc(&conversation_id, seq)
            .await
            .unwrap();

        assert_eq!(malloc_seq, seq);
    }
}
