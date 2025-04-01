use abi::{async_trait::async_trait, mongodb::results::UpdateResult, Result};
use entity::msg::{MsgDataModel, MsgDocModel};

#[async_trait]
pub trait MsgRepo: Send + Sync + 'static {
    async fn create(&self, model: MsgDocModel) -> Result<()>;
    async fn update_with_msg(
        &self,
        doc_id: &str,
        index: usize,
        value: &MsgDataModel,
    ) -> Result<UpdateResult>;
}
