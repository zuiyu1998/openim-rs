use abi::{async_trait::async_trait, Result};

#[async_trait]
pub trait SeqUserCache: Send + Sync + 'static {
    async fn set_user_read_seq_to_db(
        &self,
        conversation_id: &str,
        user_id: &str,
        seq: i64,
    ) -> Result<()>;
}
