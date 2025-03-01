use std::{collections::HashMap, sync::Arc};

use abi::{async_trait::async_trait, Result};

use crate::{
    cache::{
        redis::{new_redis_client, RedisConfig, SeqUserRedis},
        SeqUserCache,
    },
    database::mongodb::{new_mongo_database, seq_user::SeqUserMongodb},
};

use super::mongodb::MongoDbConfig;

pub struct BaseMsgTransferDatabase {
    seq_user: Arc<dyn SeqUserCache>,
}

pub struct MsgTransferDatabaseConfig {
    redis: RedisConfig,
    mongo: MongoDbConfig,
}

pub async fn new_msg_transfer_database(
    config: &MsgTransferDatabaseConfig,
) -> Result<Arc<dyn MsgTransferDatabase>> {
    let client = new_redis_client(&config.redis)?;
    let database = new_mongo_database(&config.mongo).await?;
    let seq_user_database = SeqUserMongodb::new(&database, &config.mongo.seq_user_name).await?;
    let seq_user = Arc::new(SeqUserRedis::new(client, Arc::new(seq_user_database)));

    Ok(Arc::new(BaseMsgTransferDatabase { seq_user }))
}

#[async_trait]
impl MsgTransferDatabase for BaseMsgTransferDatabase {
    async fn set_has_read_seqs(
        &self,
        conversation_id: &str,
        user_seq_map: &HashMap<String, i64>,
    ) -> Result<()> {
        for (user_id, seq) in user_seq_map.iter() {
            self.seq_user
                .set_user_read_seq_to_db(conversation_id, &user_id, *seq)
                .await?;
        }
        Ok(())
    }
}

#[async_trait]
pub trait MsgTransferDatabase: Send + Sync + 'static {
    async fn set_has_read_seqs(
        &self,
        conversation_id: &str,
        user_seq_map: &HashMap<String, i64>,
    ) -> Result<()>;
}
