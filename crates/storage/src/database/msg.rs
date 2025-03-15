use abi::{async_trait::async_trait, Result};
use entity::msg::MsgDocModel;

#[async_trait]
pub trait MsgRepo: Send + Sync + 'static {
    async fn create(&self, model: MsgDocModel) -> Result<()>;
}
