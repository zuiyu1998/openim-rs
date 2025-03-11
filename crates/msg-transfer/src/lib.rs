mod online_history_redis_consumer_handler;

use abi::{
    error::Error,
    protocol::{pb::openim_sdkws::MsgData, prost::Message},
    tokio,
    tools::{
        batcher::{Batcher, BatcherData},
        mq_producer::rdkafka::{consumer::StreamConsumer, Message as KafkaMessage},
    },
    Result,
};
use online_history_redis_consumer_handler::OnlineHistoryRedisConsumerHandler;

type HistoryBatcher = Batcher<ConsumerMessage, Error, OnlineHistoryRedisConsumerHandler>;

pub struct MsgTransferSevice {
    history_consumer: StreamConsumer,
    batcher: HistoryBatcher,
}

impl MsgTransferSevice {
    pub async fn start(self) -> Result<()> {
        let MsgTransferSevice {
            history_consumer,
            mut batcher,
        } = self;

        batcher.start().await;

        tokio::spawn(async move {
            handle_redis_message(batcher, history_consumer).await;
        });

        Ok(())
    }
}

pub async fn handle_redis_message(mut batcher: HistoryBatcher, history_consumer: StreamConsumer) {
    loop {
        match history_consumer.recv().await {
            Err(e) => {
                tracing::error!("history_consumer recv error: {}", e);
            }
            Ok(m) => {
                let key = match m.key() {
                    None => {
                        tracing::warn!("history_consumer message key not found");
                        continue;
                    }
                    Some(key) => String::from_utf8_lossy(key).to_string(),
                };

                let bytes: Vec<u8> = match m.payload_view::<[u8]>() {
                    None => {
                        tracing::warn!("history_consumer message key not found");
                        continue;
                    }
                    Some(bytes) => match bytes {
                        Err(_) => {
                            tracing::warn!("history_consumer message payload not found");
                            continue;
                        }

                        Ok(bytes) => bytes.to_vec(),
                    },
                };

                batcher.put(ConsumerMessage { key, bytes }).await;
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct ContextMessge {
    pub msg_data: MsgData,
}

impl ContextMessge {
    pub fn from_consumer_message(msg: &ConsumerMessage) -> Result<Self> {
        let msg_data = MsgData::decode(msg.bytes.as_slice())?;

        Ok(Self { msg_data })
    }
}

pub struct ConsumerMessage {
    key: String,
    bytes: Vec<u8>,
}

impl BatcherData for ConsumerMessage {
    fn key(&self) -> String {
        self.key.clone()
    }
}
