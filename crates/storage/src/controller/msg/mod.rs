use abi::{async_trait::async_trait, protocol::pb::openim_sdkws::MsgData, Result};
use tools::mq_producer::{
    kafka::{KafkaConfig, KafkaProducer},
    MQProducer,
};

pub struct BaseMsgDatabase {
    producer: Box<dyn MQProducer<Data = MsgData>>,
}

impl BaseMsgDatabase {
    pub async fn new_kafka(config: &KafkaConfig, topic_name: &str) -> Result<BaseMsgDatabase> {
        let producer = KafkaProducer::new(config, topic_name).await?;
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
