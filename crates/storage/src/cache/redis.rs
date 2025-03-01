use std::sync::Arc;

use crate::database::seq_user::SeqUserDataBase;

use super::SeqUserCache;
use abi::{async_trait::async_trait, redis, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct RedisConfig {
    host: String,
    port: u16,
}

impl RedisConfig {
    pub fn url(&self) -> String {
        format!("redis://{}:{}/", self.host, self.port)
    }
}

pub fn new_redis_client(config: &RedisConfig) -> Result< redis::Client> {
    let clinet = redis::Client::open(config.url())?;
    Ok(clinet)
}

pub struct SeqUserRedis {
    _client: redis::Client,
    seq_user_database: Arc<dyn SeqUserDataBase>,
}

impl SeqUserRedis {
    pub fn new(client: redis::Client, seq_user_database: Arc<dyn SeqUserDataBase>) -> Self {
        Self {
            _client: client,
            seq_user_database,
        }
    }
}

#[async_trait]
impl SeqUserCache for SeqUserRedis {
    async fn set_user_read_seq_to_db(
        &self,
        conversation_id: &str,
        user_id: &str,
        seq: i64,
    ) -> Result<()> {
        self.seq_user_database
            .set_user_read_seq(conversation_id, user_id, seq)
            .await?;
        Ok(())
    }
}
