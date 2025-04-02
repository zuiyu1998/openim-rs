use std::sync::Arc;

use abi::{
    protocol::{pb::openim_msg::MsgDataToMongoByMq, prost::Message as ProstMessage},
    tools::mq_producer::rdkafka::{consumer::StreamConsumer, Message},
    ErrorKind, Result,
};
use openim_storage::controller::msg_transfer::MsgTransferDatabase;

pub async fn handle_mongo_message(
    handler: OnlineHistoryMongoConsumerHandler,
    history_consumer: StreamConsumer,
) {
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

                if let Err(e) = handler.handle_chat_ws_2_mongo(&key, bytes).await {
                    tracing::error!("handle chat ws 2 mongo error: {}", e);
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct OnlineHistoryMongoConsumerHandler {
    msg_transfer_database: Arc<dyn MsgTransferDatabase>,
}

impl OnlineHistoryMongoConsumerHandler {

    pub fn new(msg_transfer_database: Arc<dyn MsgTransferDatabase>,) -> Self {
        OnlineHistoryMongoConsumerHandler { msg_transfer_database }
    }

    pub async fn handle_chat_ws_2_mongo(&self, _key: &str, bytes: Vec<u8>) -> Result<()> {
        let mut msg_from_mq: MsgDataToMongoByMq = MsgDataToMongoByMq::decode(bytes.as_slice())?;

        if msg_from_mq.msg_data.is_empty() {
            return Err(ErrorKind::MsgDataIsEmpty.into());
        }

        self.msg_transfer_database
            .batch_insert_chat_2_db(
                &msg_from_mq.conversation_id,
                &mut msg_from_mq.msg_data,
                msg_from_mq.last_seq,
            )
            .await?;

        Ok(())
    }
}
