use abi::{async_trait::async_trait, Result};

#[async_trait]
pub trait SeqConversationDataBase: Send + Sync + 'static {
    async fn malloc(&self, conversation_id: &str, size: i64) -> Result<i64>;
    async fn get_max_seq(&self, conversation_id: &str) -> Result<i64>;
}
