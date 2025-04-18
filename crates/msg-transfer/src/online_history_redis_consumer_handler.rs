use std::{collections::HashMap, sync::Arc};

use abi::{
    async_trait::async_trait,
    error::Error,
    protocol::{
        pb::{
            constant::{MsgContentType, MsgSessionType},
            msg_processor::{self, MsgOptions},
            openim_sdkws::{MarkAsReadTips, MsgData, NotificationElem},
        },
        prost::Message as ProstMessage,
    },
    serde_json,
    tools::{
        batcher::{Batcher, BatcherData, BatcherHandler, PayloadData},
        mq_producer::rdkafka::{consumer::StreamConsumer, Message},
    },
    Result,
};
use openim_storage::controller::msg_transfer::MsgTransferDatabase;

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

#[derive(Clone)]
pub struct ConsumerMessage {
    key: String,
    bytes: Vec<u8>,
}

impl BatcherData for ConsumerMessage {
    fn key(&self) -> String {
        self.key.clone()
    }
}

#[derive(Clone)]
pub struct OnlineHistoryRedisConsumerHandler {
    msg_transfer_database: Arc<dyn MsgTransferDatabase>,
    batcher: Batcher<ConsumerMessage>,
}

impl OnlineHistoryRedisConsumerHandler {
    pub fn new(
        msg_transfer_database: Arc<dyn MsgTransferDatabase>,
        batcher: Batcher<ConsumerMessage>,
    ) -> Self {
        OnlineHistoryRedisConsumerHandler {
            msg_transfer_database,
            batcher,
        }
    }

    pub async fn start(&mut self) {
        self.batcher.start(self.clone()).await;
    }

    pub async fn handle_redis_message(&mut self, history_consumer: StreamConsumer) {
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

                    self.batcher.put(ConsumerMessage { key, bytes }).await;
                }
            }
        }
    }

    pub async fn do_set_read_seq(&self, ctx_messages: &Vec<ContextMessge>) {
        let mut conversation_id: Option<String> = None;
        let mut user_seq_map: HashMap<String, i64> = Default::default();

        for message in ctx_messages.iter() {
            let content_type: MsgContentType = message.msg_data.content_type.into();
            if content_type == MsgContentType::HasReadReceipt {
                continue;
            }

            let elem: NotificationElem = match serde_json::from_slice(&message.msg_data.content) {
                Err(e) => {
                    tracing::warn!(
                        "HandlerConversationRead Unmarshal NotificationElem msg err: {}, msg: {:?}",
                        e,
                        message
                    );
                    continue;
                }
                Ok(elem) => elem,
            };

            let mut tips: MarkAsReadTips = match serde_json::from_str(&elem.detail) {
                Err(e) => {
                    tracing::warn!(
                        "HandlerConversationRead Unmarshal MarkAsReadTips msg err: {}, msg: {:?}",
                        e,
                        message
                    );
                    continue;
                }
                Ok(elem) => elem,
            };

            //The conversation ID for each batch of messages processed by the batcher is the same.

            conversation_id = Some(tips.conversation_id);

            //记录seqs中最大值
            if tips.seqs.len() > 0 {
                for seq in tips.seqs {
                    if tips.has_read_seq < seq {
                        tips.has_read_seq = seq;
                    }

                    tips.seqs = vec![];
                }
            }

            if tips.has_read_seq < 0 {
                continue;
            }

            let mark_as_read_user_id = match user_seq_map.get(&tips.mark_as_read_user_id) {
                None => {
                    continue;
                }
                Some(mark_as_read_user_id) => *mark_as_read_user_id,
            };

            if mark_as_read_user_id > tips.has_read_seq {
                continue;
            }

            user_seq_map.insert(tips.mark_as_read_user_id, tips.has_read_seq);
        }

        if user_seq_map.is_empty() || conversation_id.is_none() {
            return;
        }

        let conversation_id = conversation_id.unwrap();

        if let Err(e) = self
            .msg_transfer_database
            .set_has_read_seqs(&conversation_id, &user_seq_map)
            .await
        {
            tracing::error!(
                "set read seq to db error: {}, conversation_id: {}, user_seq_map: {:?}",
                e,
                conversation_id,
                user_seq_map
            );
        }
    }

    pub async fn to_push_topic(
        &self,
        key: &str,
        conversation_id: &str,
        msg_list: Vec<ContextMessge>,
    ) {
        for msg in msg_list.into_iter() {
            if let Err(e) = self
                .msg_transfer_database
                .msg_to_push_mq(key, conversation_id, msg.msg_data)
                .await
            {
                tracing::error!("msg_to_push_mq error is: {}", e);
            }
        }
    }

    pub async fn handle_msg(
        &self,
        key: &str,
        conversation_id: &str,
        storage_msg_list: Vec<ContextMessge>,
        not_storage_msg_list: Vec<ContextMessge>,
    ) {
        tracing::info!("handle storage msg");

        for msg in storage_msg_list.iter() {
            tracing::debug!("handle storage msg, msg is: {:?}", msg.msg_data);
        }

        self.to_push_topic(key, conversation_id, not_storage_msg_list)
            .await;

        let mut storage_message_list: Vec<MsgData> = storage_msg_list
            .iter()
            .map(|ctx| ctx.msg_data.clone())
            .collect();

        if storage_message_list.is_empty() {
            return;
        }

        let msg = storage_message_list[0].clone();

        let (last_seq, new_conversation, user_seq_map) = match self
            .msg_transfer_database
            .batch_insert_chat_2_cache(conversation_id, &mut storage_message_list)
            .await
        {
            Err(e) => {
                tracing::error!("batch data insert to redis err: {}", e);

                return;
            }
            Ok(res) => res,
        };

        tracing::info!("batch_insert_chat_2_cache end");

        if let Err(e) = self
            .msg_transfer_database
            .set_has_read_seqs(conversation_id, &user_seq_map)
            .await
        {
            //todo 服务监控
            tracing::error!("set_has_read_seqs error: {}", e);
        }

        if new_conversation {
            match MsgSessionType::from(msg.session_type) {
                MsgSessionType::SingleChatType | MsgSessionType::NotificationChatType => {}
                _ => {
                    tracing::warn!("unknow session type: {}", msg.session_type);
                }
            }
        }

        tracing::info!("success incr to next topic");

        if let Err(e) = self
            .msg_transfer_database
            .msg_to_mongo_mq(key, conversation_id, storage_message_list, last_seq)
            .await
        {
            tracing::error!(
                "msg_to_mongo_mq error: {}, args: conversation_id: {}, ",
                e,
                conversation_id
            );
        }

        self.to_push_topic(key, conversation_id, storage_msg_list)
            .await;
        tracing::info!("to_push_topic end");
    }
}

#[async_trait]
impl BatcherHandler for OnlineHistoryRedisConsumerHandler {
    type Data = ConsumerMessage;
    type Error = Error;

    async fn handle(&self, key: String, payload: PayloadData<Self::Data>) -> Result<(), Error> {
        let ctx_messages = get_context_messges(&payload);
        tracing::info!(
            "msg arrived, msgList length: {}, key: {}",
            ctx_messages.len(),
            key
        );

        self.do_set_read_seq(&ctx_messages).await;

        let (
            storage_msg_list,
            not_storage_msg_list,
            _storage_notification_list,
            _not_storage_notification_list,
        ) = categorize_message_lists(&ctx_messages);

        let conversation_id_msgg =
            msg_processor::get_chat_conversation_id_by_msg(&ctx_messages[0].msg_data);
        let _conversation_id_notification =
            msg_processor::get_notification_conversation_id_by_msg(&ctx_messages[0].msg_data);

        self.handle_msg(
            &payload.key,
            &conversation_id_msgg,
            storage_msg_list,
            not_storage_msg_list,
        )
        .await;

        Ok(())
    }
}

fn categorize_message_lists(
    ctx_messages: &Vec<ContextMessge>,
) -> (
    Vec<ContextMessge>,
    Vec<ContextMessge>,
    Vec<ContextMessge>,
    Vec<ContextMessge>,
) {
    let mut storage_msg_list = vec![];
    let mut not_storage_msg_list = vec![];
    let mut storage_notification_list = vec![];
    let mut not_storage_notification_list = vec![];

    for message in ctx_messages.iter() {
        let mut message = message.clone();

        let (notification, send_msg, history) = {
            let message_options = MsgOptions::from_msg_data(&mut message.msg_data);
            let notification = message_options.is_notification();
            let send_msg = message_options.is_send_msg();
            let history = message_options.is_history();
            (notification, send_msg, history)
        };

        if !notification {
            if send_msg {
                let mut msg = message.msg_data.clone();

                let mut message_options = MsgOptions::from_msg_data(&mut message.msg_data);
                let offline_push = message_options.is_offline_push();
                let unread_count = message_options.is_unread_count();

                msg.options = msg_processor::new_msg_options();

                let mut msg_options = MsgOptions::from_msg_data(&mut msg);
                msg_options.set_offline_push(offline_push);
                msg_options.set_unread_count(unread_count);

                message_options.set_offline_push(false);
                message_options.set_unread_count(false);

                storage_msg_list.push(ContextMessge { msg_data: msg });
            }

            if history {
                storage_notification_list.push(message.clone());
            } else {
                not_storage_notification_list.push(message.clone());
            }
        } else {
            if history {
                storage_msg_list.push(message.clone());
            } else {
                not_storage_msg_list.push(message.clone());
            }
        }
    }

    (
        storage_msg_list,
        not_storage_msg_list,
        storage_notification_list,
        not_storage_notification_list,
    )
}

fn get_context_messges(payload: &PayloadData<ConsumerMessage>) -> Vec<ContextMessge> {
    let mut messages = vec![];

    for message in payload.get_data().iter() {
        match ContextMessge::from_consumer_message(&message) {
            Err(e) => {
                tracing::error!("Parse ConsumerMessage fail, error: {}", e);
            }
            Ok(msg) => {
                messages.push(msg);
            }
        }
    }

    messages
}
