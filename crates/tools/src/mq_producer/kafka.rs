use super::{MQError, MQProducer};
use std::{marker::PhantomData, time::Duration};

use rdkafka::{
    admin::{AdminClient, AdminOptions, NewTopic, TopicReplication},
    client::DefaultClientContext,
    error::KafkaError,
    producer::{FutureProducer, FutureRecord},
    ClientConfig,
};
use serde::{Deserialize, Serialize};
use tracing::info;
use prost::Message;
use async_trait::async_trait;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KafkaConfig {
    pub name: Option<String>,
    pub password: Option<String>,
    pub connect_timeout: u16,
    pub timeout: u16,
    pub to_redis_topic: String,
    pub broker: String,
}

pub struct KafkaBuilder<'a> {
    pub config: &'a KafkaConfig,
}

impl<'a> KafkaBuilder<'a> {
    pub async fn get_future_producer(
        &self,
        topic_name: &str,
    ) -> Result<FutureProducer, KafkaError> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", &self.config.broker)
            .set("message.timeout.ms", &self.config.timeout.to_string())
            .set(
                "socket.timeout.ms",
                &self.config.connect_timeout.to_string(),
            )
            // .set("acks", config.kafka.producer.acks.clone())
            // make sure the message is sent exactly once
            .set("enable.idempotence", "true")
            // .set("retries", config.kafka.producer.max_retry.to_string())
            // .set(
            //     "retry.backoff.ms",
            //     config.kafka.producer.retry_interval.to_string(),
            // )
            .create()
            .expect("Producer creation error");

        Self::ensure_topic_exists(topic_name, &self.config.broker, self.config.connect_timeout)
            .await?;

        Ok(producer)
    }

    pub async fn ensure_topic_exists(
        topic_name: &str,
        brokers: &str,
        timeout: u16,
    ) -> Result<(), KafkaError> {
        let admin_client: AdminClient<DefaultClientContext> = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("socket.timeout.ms", timeout.to_string())
            .create()?;

        let new_topics = [NewTopic {
            name: topic_name,
            num_partitions: 1,
            replication: TopicReplication::Fixed(1),
            config: vec![],
        }];

        let options = AdminOptions::new();
        admin_client.create_topics(&new_topics, &options).await?;

        match admin_client.create_topics(&new_topics, &options).await {
            Ok(_) => {
                info!("Topic not exist; create '{}' ", topic_name);
                Ok(())
            }
            Err(KafkaError::AdminOpCreation(_)) => {
                info!("Topic '{}' already exists.", topic_name);
                Ok(())
            }
            Err(err) => Err(err.into()),
        }
    }
}

pub struct KafkaProducer<T> {
    topic: String,
    producer: FutureProducer,
    _marker: PhantomData<T>
}

impl<T> KafkaProducer<T> {
    pub async fn new(config: &KafkaConfig) -> Result<Self, MQError> {
        let builder = KafkaBuilder { config };
        let producer = builder.get_future_producer(&config.to_redis_topic).await?;

        Ok(Self {
            topic: config.to_redis_topic.to_string(),
            producer,
            _marker: Default::default()
        })
    }
}

#[async_trait]
impl<T: Message + 'static> MQProducer for KafkaProducer<T> {

    type Data = T;

    async fn msg_to_mq(&self, key: &str, msg_data: &T) -> Result<(), MQError> {
        let mut payload: Vec<u8> = vec![];
        msg_data.encode(&mut payload)?;

        let record = FutureRecord::to(&self.topic).key(key).payload(&payload);

        //todo header

        if let Err((e, _)) = self.producer.send(record, Duration::from_secs(0)).await {
            return Err(e.into());
        };

        Ok(())
    }
}
