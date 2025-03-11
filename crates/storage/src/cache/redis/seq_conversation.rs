use std::{sync::Arc, time::Duration};

use crate::{cache::SeqConversationCache, database::seq_conversation::SeqConversationDataBase};

use abi::{
    async_trait::async_trait, protocol::pb::msg_processor, redis, utils::time_util, ErrorKind,
    Result,
};

use super::consts::get_malloc_seq_key;

pub struct SeqConversationRedis {
    client: redis::Client,
    lock_time: Duration,
    data_time: Duration,
    seq_conversation_database: Arc<dyn SeqConversationDataBase>,
}

fn get_seq_malloc_key(conversation_id: &str) -> String {
    get_malloc_seq_key(conversation_id)
}

impl SeqConversationRedis {
    pub fn new(
        client: redis::Client,
        lock_time: Duration,
        data_time: Duration,
        seq_conversation_database: Arc<dyn SeqConversationDataBase>,
    ) -> SeqConversationRedis {
        SeqConversationRedis {
            client,
            lock_time,
            data_time,
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
        local curr_seq = tonumber(ARGV[3])
        local last_seq = tonumber(ARGV[4])
        local mallocTime = ARGV[5]
        if redis.call("EXISTS", key) == 0 then
            redis.call("HSET", key, "CURR", curr_seq, "LAST", last_seq, "TIME", mallocTime)
            redis.call("EXPIRE", key, dataSecond)
            return 1
        end
        if redis.call("HGET", key, "LOCK") ~= lockValue then
            return 2
        end
        redis.call("HDEL", key, "LOCK")
        redis.call("HSET", key, "CURR", curr_seq, "LAST", last_seq, "TIME", mallocTime)
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
            return Err(ErrorKind::SizeIsSmall.into());
        }

        let key = get_seq_malloc_key(conversation_id);

        for _ in 0..10 {
            let states = self._malloc(&key, size).await?;

            match states[0] {
                //success
                0 => {
                    return Ok((states[1], states[3]));
                }
                // not found
                1 => {
                    let malloc_size = self.get_malloc_size(conversation_id, size);
                    let seq = self
                        .seq_conversation_database
                        .malloc(conversation_id, size)
                        .await?;

                    self.set_seq_retry(&key, states[1], seq + size, seq + malloc_size, states[2])
                        .await;

                    return Ok((states[1], states[3]));
                }
                //locked
                2 => {
                    todo!()
                }
                // exceeded cache max value
                3 => {
                    let curr_seq = states[1];
                    let last_seq = states[2];
                    let mill = states[4];
                    let malloc_size = self.get_malloc_size(conversation_id, size);
                    let seq = self
                        .seq_conversation_database
                        .malloc(conversation_id, malloc_size)
                        .await?;

                    if last_seq == seq {
                        self.set_seq_retry(
                            &key,
                            states[3],
                            curr_seq + size,
                            seq + malloc_size,
                            mill,
                        )
                        .await;
                    } else {
                        tracing::warn!(
                            "malloc seq not equal cache last seq. args is: conversation_id: {}, curr_seq: {}, last_seq: {}, seq: {}",
                            conversation_id, curr_seq,last_seq,seq
                        );
                        return Ok((seq, mill));
                    }
                }
                _ => {
                    tracing::error!(
                        "malloc seq unknown state.state: {}, conversation_id: {}, size: {}",
                        states[0],
                        conversation_id,
                        size
                    );
                    return Err(ErrorKind::MallocUnknownState(states[0]).into());
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

    async fn _malloc(&self, key: &str, size: i64) -> Result<Vec<i64>> {
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
            local curr_seq = tonumber(redis.call("HGET", key, "CURR"))
            local last_seq = tonumber(redis.call("HGET", key, "LAST"))
            if size == 0 then
                redis.call("EXPIRE", key, dataSecond)
                table.insert(result, 0)
                table.insert(result, curr_seq)
                table.insert(result, last_seq)
                local setTime = redis.call("HGET", key, "TIME")
                if setTime then
                    table.insert(result, setTime)	
                else
                    table.insert(result, 0)
                end
                return result
            end
            local max_seq = curr_seq + size
            if max_seq > last_seq then
                local lockValue = math.random(0, 999999999)
                redis.call("HSET", key, "LOCK", lockValue)
                redis.call("HSET", key, "CURR", last_seq)
                redis.call("HSET", key, "TIME", mallocTime)
                redis.call("EXPIRE", key, lockSecond)
                table.insert(result, 3)
                table.insert(result, curr_seq)
                table.insert(result, last_seq)
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
        Ok(result)
    }
}

#[async_trait]
impl SeqConversationCache for SeqConversationRedis {
    async fn malloc(&self, conversation_id: &str, size: i64) -> Result<i64> {
        let (seq, _) = self.malloc_time(conversation_id, size).await?;
        Ok(seq)
    }
}
