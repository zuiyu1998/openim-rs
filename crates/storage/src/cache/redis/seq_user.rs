use std::sync::Arc;

use crate::{cache::seq_user::SeqUserCache, database::seq_user::SeqUserRepo};

use abi::{async_trait::async_trait, redis, Result};

pub struct SeqUserRedis {
    _client: redis::Client,
    seq_user_repo: Arc<dyn SeqUserRepo>,
}

impl SeqUserRedis {
    pub fn new(client: redis::Client, seq_user_repo: Arc<dyn SeqUserRepo>) -> Self {
        Self {
            _client: client,
            seq_user_repo,
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
        self.seq_user_repo
            .set_user_read_seq(conversation_id, user_id, seq)
            .await?;
        Ok(())
    }
}
