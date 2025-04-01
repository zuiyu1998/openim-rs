use std::{collections::HashMap, sync::Arc};

use abi::{
    async_trait::async_trait,
    protocol::pb::{
        constant::constant,
        openim_msg::{MsgDataToMongoByMq, PushMsgDataToMq},
        openim_sdkws::MsgData,
    },
    Result,
};
use entity::msg::{MsgDataModel, MsgDocModel, MsgInfoModel};
use tools::mq_producer::MQProducer;

use crate::{
    cache::{msg::MsgCache, SeqConversationCache, SeqUserCache},
    database::msg::MsgRepo,
};

pub struct CommonMsgTransferDatabase {
    seq_user_cache: Arc<dyn SeqUserCache>,
    seq_conversation_cache: Arc<dyn SeqConversationCache>,
    msg_cache: Arc<dyn MsgCache>,
    producer_to_push: Box<dyn MQProducer<Data = PushMsgDataToMq>>,
    producer_to_mongo: Box<dyn MQProducer<Data = MsgDataToMongoByMq>>,
    msg_repo: Arc<dyn MsgRepo>,
}

impl CommonMsgTransferDatabase {
    pub fn new(
        seq_user_cache: Arc<dyn SeqUserCache>,
        seq_conversation_cache: Arc<dyn SeqConversationCache>,
        msg_cache: Arc<dyn MsgCache>,
        producer_to_push: Box<dyn MQProducer<Data = PushMsgDataToMq>>,
        producer_to_mongo: Box<dyn MQProducer<Data = MsgDataToMongoByMq>>,
        msg_repo: Arc<dyn MsgRepo>,
    ) -> Self {
        Self {
            seq_user_cache,
            seq_conversation_cache,
            msg_cache,
            producer_to_push,
            producer_to_mongo,
            msg_repo,
        }
    }

    pub async fn batch_insert_block_with_msgs(
        &self,
        conversation_id: &str,
        msgs: Vec<MsgDataModel>,
        first_seq: i64,
    ) -> Result<()> {
        //todo 检查消息的有效性

        let num = MsgDocModel::get_single_goc_msg_num();

        let len = msgs.len();
        let mut i = 0;
        let mut try_update = true;

        while i < len {
            let seq = first_seq + i as i64;
            if try_update {
                let matched = self
                    .update_with_msg_model(conversation_id, seq, &msgs[i])
                    .await?;

                if matched {
                    continue;
                }
            }
            let mut doc = MsgDocModel {
                doc_id: MsgDocModel::get_doc_id(conversation_id, seq),
                msgs: vec![MsgInfoModel::default(); num as usize],
            };

            let mut insert_count = 0;

            for j in i..len {
                let seq = first_seq + j as i64;

                if MsgDocModel::get_doc_id(conversation_id, seq) != doc.doc_id {
                    break;
                }

                insert_count += 1;

                doc.msgs[MsgDocModel::get_msg_index(seq) as usize] =
                    MsgInfoModel::from_msg_data_model(msgs[j].clone());
            }

            match self.msg_repo.create(doc).await {
                Ok(_) => {
                    try_update = false;
                    i += insert_count - 1;
                }
                Err(e) if e.is_mongodb_duplicate_key_error() => {
                    i -= 1;
                    try_update = true;
                    continue;
                }
                Err(e) => {
                    return Err(e);
                }
            };
        }

        Ok(())
    }

    pub async fn update_with_msg_model(
        &self,
        conversation_id: &str,
        seq: i64,
        value: &MsgDataModel,
    ) -> Result<bool> {
        let doc_id = MsgDocModel::get_doc_id(conversation_id, seq);
        let index = MsgDocModel::get_msg_index(seq) as usize;
        let res = self.msg_repo.update_with_msg(&doc_id, index, value).await?;
        Ok(res.matched_count > 0)
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
            self.seq_user_cache
                .set_user_read_seq_to_db(conversation_id, &user_id, *seq)
                .await?;
        }
        Ok(())
    }

    async fn batch_insert_chat_2_db(
        &self,
        conversation_id: &str,
        message_list: &mut Vec<MsgData>,
        current_max_seq: i64,
    ) -> Result<()> {
        let mut msgs = vec![];
        let mut seqs = vec![];

        for message in message_list.iter_mut() {
            seqs.push(message.seq);

            if message.status == constant::MSG_STATUS_SENDING {
                message.status = constant::MSG_STATUS_SEND_SUCCESS;
            }

            msgs.push(MsgDataModel::from_msg_data(message));
        }

        self.batch_insert_block_with_msgs(conversation_id, msgs, current_max_seq)
            .await?;
        todo!()
    }

    async fn batch_insert_chat_2_cache(
        &self,
        conversation_id: &str,
        message_list: &mut Vec<MsgData>,
    ) -> Result<(i64, bool, HashMap<String, i64>)> {
        //todo msg table

        let mut current_max_seq = self
            .seq_conversation_cache
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

    async fn batch_insert_chat_2_db(
        &self,
        conversation_id: &str,
        message_list: &mut Vec<MsgData>,
        current_max_seq: i64,
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
