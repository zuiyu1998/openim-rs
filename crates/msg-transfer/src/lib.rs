mod online_history_redis_consumer_handler;
mod online_msg_to_mongo_handler;

use abi::{
    config::MQTopcis,
    tokio,
    tools::{
        batcher::{Batcher, BatcherConfig},
        mq_producer::{
            kafka::{KafkaBuilder, KafkaConfig},
            rdkafka::consumer::StreamConsumer,
        },
    },
    Result,
};
use online_history_redis_consumer_handler::{handle_redis_message, HistoryBatcher};
use online_msg_to_mongo_handler::{handle_mongo_message, OnlineHistoryMongoConsumerHandler};

pub struct MsgTransferSeviceConfig {
    kafka: KafkaConfig,
    topisc: MQTopcis,
    batcher: BatcherConfig,
}

pub struct MsgTransferSevice {
    history_consumer: StreamConsumer,
    //历史消息mongo存储
    history_mongo_consumer: StreamConsumer,
    batcher: HistoryBatcher,
    history_mongo_consumer_handler: OnlineHistoryMongoConsumerHandler,
}

impl MsgTransferSevice {
    pub fn new(
        history_consumer: StreamConsumer,
        history_mongo_consumer: StreamConsumer,
        batcher: HistoryBatcher,
        history_mongo_consumer_handler: OnlineHistoryMongoConsumerHandler,
    ) -> Self {
        Self {
            history_consumer,
            history_mongo_consumer,
            batcher,
            history_mongo_consumer_handler,
        }
    }

    pub async fn start(config: &MsgTransferSeviceConfig) -> Result<()> {
        let kafka_builder = KafkaBuilder::new(&config.kafka);

        let history_consumer = kafka_builder
            .get_stream_consumer(&config.topisc.to_redis_topic)
            .await?;

        let history_mongo_consumer = kafka_builder
            .get_stream_consumer(&config.topisc.to_mongo_topic)
            .await?;

        //    let batcher = Batcher::new(&config.batcher, handler);

        todo!()
    }

    pub async fn run(self) -> Result<()> {
        let MsgTransferSevice {
            history_consumer,
            mut batcher,
            history_mongo_consumer_handler,
            history_mongo_consumer,
        } = self;

        batcher.start().await;

        tokio::spawn(async move {
            handle_redis_message(batcher, history_consumer).await;
        });

        tokio::spawn(async move {
            handle_mongo_message(history_mongo_consumer_handler, history_mongo_consumer).await;
        });

        Ok(())
    }
}
