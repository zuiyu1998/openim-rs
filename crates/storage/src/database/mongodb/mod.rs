pub mod seq_user;
pub mod seq_conversation;
pub mod msg;

use abi::{
    mongodb::{Client, Database},
    Result,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MongoDbConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
    pub seq_user_name: String,
    pub seq_conversation_name: String,
}

impl MongoDbConfig {
    pub fn server_url(&self) -> String {
        match (self.user.is_empty(), self.password.is_empty()) {
            (true, _) => {
                format!("mongodb://{}:{}", self.host, self.port)
            }
            (false, true) => {
                format!("mongodb://{}@{}:{}", self.user, self.host, self.port)
            }
            (false, false) => {
                format!(
                    "mongodb://{}:{}@{}:{}",
                    self.user, self.password, self.host, self.port
                )
            }
        }
    }

    pub fn url(&self) -> String {
        format!("{}/{}", self.server_url(), self.database)
    }
}

pub async fn new_mongo_database(config: &MongoDbConfig) -> Result<Database> {
    let db = Client::with_uri_str(config.url())
        .await?
        .database(&config.database);
    Ok(db)
}
