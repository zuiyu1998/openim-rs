pub mod kafka;

pub use rdkafka;

use async_trait::async_trait;
use thiserror::Error;
use rdkafka::error::KafkaError;
use prost::EncodeError as ProstEncodeError;

#[derive(Debug, Error)]
pub enum MQError {
    #[error("kafka error: {0}")]
    KafkaError(#[from] KafkaError),
    #[error("prost encode error: {0}")]
    ProstEncodeError(#[from] ProstEncodeError),
}

#[async_trait]
pub trait MQProducer: Sync + Send + 'static {
    type Data;

    async fn msg_to_mq(&self, key: &str, msg_data: &Self::Data) -> Result<(), MQError>;
}
