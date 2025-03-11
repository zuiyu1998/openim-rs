use std::{collections::HashMap, sync::Arc};

use abi::{
    async_trait::async_trait,
    protocol::pb::{
        openim_msg::{MsgDataToMongoByMq, PushMsgDataToMq},
        openim_sdkws::MsgData,
    },
    Result,
};
use tools::mq_producer::MQProducer;

use crate::{
    cache::{msg::MsgCache, SeqConversationCache, SeqUserCache},
    model::MsgInfoModel,
};

pub struct CommonMsgTransferDatabase {
    seq_user: Arc<dyn SeqUserCache>,
    seq_conversation: Arc<dyn SeqConversationCache>,
    msg_cache: Arc<dyn MsgCache>,
    producer_to_push: Box<dyn MQProducer<Data = PushMsgDataToMq>>,
    producer_to_mongo: Box<dyn MQProducer<Data = MsgDataToMongoByMq>>,
}

impl CommonMsgTransferDatabase {
    pub fn new(
        seq_user: Arc<dyn SeqUserCache>,
        seq_conversation: Arc<dyn SeqConversationCache>,
        msg_cache: Arc<dyn MsgCache>,
        producer_to_push: Box<dyn MQProducer<Data = PushMsgDataToMq>>,
        producer_to_mongo: Box<dyn MQProducer<Data = MsgDataToMongoByMq>>,
    ) -> Self {
        Self {
            seq_user,
            seq_conversation,
            msg_cache,
            producer_to_push,
            producer_to_mongo,
        }
    }
}

#[async_trait]
impl MsgTransferDatabase for CommonMsgTransferDatabase {
    async fn set_has_read_seqs(
        &self,
        conversation_id: &str,
        user_seq_map: &HashMap<String, i64>,
    ) -> Result<()> {
        for (user_id, seq) in user_seq_map.iter() {
            self.seq_user
                .set_user_read_seq_to_db(conversation_id, &user_id, *seq)
                .await?;
        }
        Ok(())
    }

    async fn batch_insert_chat_2_cache(
        &self,
        conversation_id: &str,
        message_list: &mut Vec<MsgData>,
    ) -> Result<(i64, bool, HashMap<String, i64>)> {
        //todo msg table

        let mut current_max_seq = self
            .seq_conversation
            .malloc(conversation_id, message_list.len() as i64)
            .await?;

        let new = current_max_seq == 0;
        let last_max_seq = current_max_seq;
        let mut user_seq_map: HashMap<String, i64> = HashMap::default();
        let mut seqs = vec![];

        for msg in message_list.iter_mut() {
            current_max_seq += 1;
            msg.seq = current_max_seq;
            user_seq_map.insert(msg.send_id.clone(), msg.seq);
            seqs.push(msg.seq);
        }

        let msg_models = message_list
            .iter()
            .map(|msg_data| MsgInfoModel::from_msg_data(msg_data))
            .collect::<Vec<MsgInfoModel>>();

        self.msg_cache
            .set_message_by_seqs(conversation_id, msg_models)
            .await?;

        return Ok((last_max_seq, new, user_seq_map));
    }

    async fn msg_to_push_mq(
        &self,
        key: &str,
        conversation_id: &str,
        msg2mq: MsgData,
    ) -> Result<()> {
        let msg_data = PushMsgDataToMq {
            conversation_id: conversation_id.to_owned(),
            msg_data: Some(msg2mq),
            ..Default::default()
        };

        self.producer_to_push.msg_to_mq(key, &msg_data).await?;

        Ok(())
    }

    async fn msg_to_mongo_mq(
        &self,
        key: &str,
        conversation_id: &str,
        msgs: Vec<MsgData>,
        last_seq: i64,
    ) -> Result<()> {
        let msg_data = MsgDataToMongoByMq {
            last_seq,
            conversation_id: conversation_id.to_owned(),
            msg_data: msgs,
            ..Default::default()
        };

        self.producer_to_mongo.msg_to_mq(key, &msg_data).await?;

        Ok(())
    }
}

#[async_trait]
pub trait MsgTransferDatabase: Send + Sync + 'static {
    async fn set_has_read_seqs(
        &self,
        conversation_id: &str,
        user_seq_map: &HashMap<String, i64>,
    ) -> Result<()>;

    async fn batch_insert_chat_2_cache(
        &self,
        conversation_id: &str,
        message_list: &mut Vec<MsgData>,
    ) -> Result<(i64, bool, HashMap<String, i64>)>;

    async fn msg_to_push_mq(&self, key: &str, conversation_id: &str, msg2mq: MsgData)
        -> Result<()>;

    async fn msg_to_mongo_mq(
        &self,
        key: &str,
        conversation_id: &str,
        msgs: Vec<MsgData>,
        last_seq: i64,
    ) -> Result<()>;
}
