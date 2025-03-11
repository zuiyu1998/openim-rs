use abi::{async_trait::async_trait, Result};

#[async_trait]
pub trait SeqConversationCache: Send + Sync + 'static {
    async fn malloc(&self, key: &str, size: i64) -> Result<i64>;
}
