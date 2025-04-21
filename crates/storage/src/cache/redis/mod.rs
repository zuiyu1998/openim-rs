pub mod consts;
pub mod msg;
pub mod seq_conversation;
pub mod seq_user;

pub use seq_user::SeqUserRedis;

use abi::{redis, Result};
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

pub fn new_redis_client(config: &RedisConfig) -> Result<redis::Client> {
    let clinet = redis::Client::open(config.url())?;
    Ok(clinet)
}
