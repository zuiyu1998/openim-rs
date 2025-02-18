use abi::{config::KafkaConfig, protocol::pb::openim_sdkws::MsgData, tonic::async_trait, Result};
use tools::mq_producer::{kafka::KafkaProducer, MQProducer};

pub struct BaseMsgDatabase {
    producer: Box<dyn MQProducer>,
}

impl BaseMsgDatabase {
    pub async fn new_kafka(config: &KafkaConfig) -> Result<BaseMsgDatabase> {
        let producer = KafkaProducer::new(config).await?;
        Ok(BaseMsgDatabase {
            producer: Box::new(producer),
        })
    }
}

#[async_trait]
impl MsgDatabase for BaseMsgDatabase {
    async fn msg_to_mq(&self, key: &str, msg_2_data: &MsgData) -> Result<()> {
        self.producer.msg_to_mq(key, msg_2_data).await?;

        Ok(())
    }
}

#[async_trait]
pub trait MsgDatabase: Send + Sync + 'static {
    async fn msg_to_mq(&self, key: &str, msg_2_data: &MsgData) -> Result<()>;
}
