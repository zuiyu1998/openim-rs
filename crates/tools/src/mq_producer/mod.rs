pub mod kafka;

use abi::{protocol::{pb::openim_sdkws::MsgData, tonic::async_trait}, Result};

#[async_trait]
pub trait MQProducer: Sync + Send + 'static {
    async fn msg_to_mq(&self, key: &str, msg_data: &MsgData) -> Result<()>;
}
