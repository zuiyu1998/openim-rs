use abi::{async_trait::async_trait, Result};

use entity::msg::MsgInfoModel;

#[async_trait]
pub trait MsgCache: Send + Sync + 'static {
    async fn set_message_by_seqs(
        &self,
        conversation_id: &str,
        msgs: Vec<MsgInfoModel>,
    ) -> Result<()>;
}
