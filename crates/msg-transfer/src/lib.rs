mod online_history_redis_consumer_handler;
mod online_msg_to_mongo_handler;

use std::sync::Arc;

use abi::{
    config::MQTopcis,
    tokio::{self},
    tools::{
        batcher::{Batcher, BatcherConfig},
        mq_producer::{
            kafka::{KafkaBuilder, KafkaConfig, KafkaProducer},
            rdkafka::consumer::StreamConsumer,
        },
    },
    Result,
};
use online_history_redis_consumer_handler::OnlineHistoryRedisConsumerHandler;
use online_msg_to_mongo_handler::{handle_mongo_message, OnlineHistoryMongoConsumerHandler};
use openim_storage::{
    cache::redis::{
        msg::MsgCacheRedis, new_redis_client, seq_conversation::SeqConversationRedis, RedisConfig,
        SeqUserRedis,
    },
    controller::msg_transfer::CommonMsgTransferDatabase,
    database::mongodb::{
        msg::MsgMongodb, new_mongo_database, seq_conversation::SeqConversationMongodb,
        seq_user::SeqUserMongodb, MongodbConfig,
    },
};

pub struct MsgTransferSeviceConfig {
    pub kafka: KafkaConfig,
    pub topics: MQTopcis,
    pub batcher: BatcherConfig,
    pub redis: RedisConfig,
    pub mongo: MongodbConfig,
}

pub struct MsgTransferSevice {
    history_redis_consumer: StreamConsumer,
    //历史消息mongo存储
    history_mongo_consumer: StreamConsumer,
    history_redis_consumer_handler: OnlineHistoryRedisConsumerHandler,
    history_mongo_consumer_handler: OnlineHistoryMongoConsumerHandler,
}

impl MsgTransferSevice {
    pub fn new(
        history_redis_consumer: StreamConsumer,
        history_mongo_consumer: StreamConsumer,
        history_redis_consumer_handler: OnlineHistoryRedisConsumerHandler,
        history_mongo_consumer_handler: OnlineHistoryMongoConsumerHandler,
    ) -> Self {
        Self {
            history_redis_consumer,
            history_mongo_consumer,
            history_redis_consumer_handler,
            history_mongo_consumer_handler,
        }
    }

    pub async fn start(config: &MsgTransferSeviceConfig) -> Result<()> {
        let kafka_builder = KafkaBuilder::new(&config.kafka);

        let history_redis_consumer = kafka_builder
            .get_stream_consumer(
                &config.topics.to_redis_topic,
                &config.topics.to_redis_topic_group_id,
            )
            .await?;

        let history_mongo_consumer = kafka_builder
            .get_stream_consumer(
                &config.topics.to_mongo_topic,
                &config.topics.to_mongo_topic_group_id,
            )
            .await?;

        let batcher = Batcher::new(&config.batcher);

        let redis_client = new_redis_client(&config.redis)?;

        let database = new_mongo_database(&config.mongo).await?;

        let seq_user_database =
            Arc::new(SeqUserMongodb::new(&database, &config.mongo.seq_user_name).await?);
        let seq_conversation_database = Arc::new(
            SeqConversationMongodb::new(&database, &config.mongo.seq_conversation_name).await?,
        );

        let seq_user_cache = Arc::new(SeqUserRedis::new(redis_client.clone(), seq_user_database));
        let seq_conversation_cache = Arc::new(SeqConversationRedis::new(
            redis_client.clone(),
            seq_conversation_database,
        ));

        let msg_cache = Arc::new(MsgCacheRedis::new(redis_client.clone()));

        let producer_to_push =
            Box::new(KafkaProducer::new(&config.kafka, &config.topics.to_push_topic).await?);
        let producer_to_mongo =
            Box::new(KafkaProducer::new(&config.kafka, &config.topics.to_mongo_topic).await?);

        let msg_repo = Arc::new(MsgMongodb::new(&database));

        let msg_transfer_database = Arc::new(CommonMsgTransferDatabase::new(
            seq_user_cache,
            seq_conversation_cache,
            msg_cache,
            producer_to_push,
            producer_to_mongo,
            msg_repo,
        ));

        let history_redis_consumer_handler =
            OnlineHistoryRedisConsumerHandler::new(msg_transfer_database.clone(), batcher);

        let history_mongo_consumer_handler =
            OnlineHistoryMongoConsumerHandler::new(msg_transfer_database);

        let msg_transfer_service = MsgTransferSevice::new(
            history_redis_consumer,
            history_mongo_consumer,
            history_redis_consumer_handler,
            history_mongo_consumer_handler,
        );

        msg_transfer_service.run().await?;
        Ok(())
    }

    pub async fn run(self) -> Result<()> {
        let MsgTransferSevice {
            history_redis_consumer,
            history_mongo_consumer,
            mut history_redis_consumer_handler,
            history_mongo_consumer_handler,
        } = self;

        history_redis_consumer_handler.start().await;

        let mut tasks = Vec::with_capacity(2);

        let redis_task = tokio::spawn(async move {
            history_redis_consumer_handler
                .handle_redis_message(history_redis_consumer)
                .await;
        });

        tasks.push(redis_task);

        let mongo_task = tokio::spawn(async move {
            handle_mongo_message(history_mongo_consumer_handler, history_mongo_consumer).await;
        });

        tasks.push(mongo_task);

        futures::future::try_join_all(tasks).await?;

        Ok(())
    }
}
