use abi::{async_trait::async_trait, Result};

#[async_trait]
pub trait SeqUserRepo: Send + Sync + 'static {
   async fn set_user_read_seq(&self, conversation_id: &str, user_id: &str, seq: i64) -> Result<()>;
   async fn get_user_read_seq(&self, conversation_id: &str, user_id: &str) -> Result<i64>;
}