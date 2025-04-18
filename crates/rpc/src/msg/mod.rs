mod rpc;

use std::sync::Arc;

use abi::{
    config::{MQTopcis, Share},
    encrypt::md5,
    protocol::pb::{
        constant::{constant, MsgContentType, MsgSessionType},
        conversation_util,
        msg_processor::MsgOptions,
        openim_msg::{msg_server::MsgServer, SendMsgReq, SendMsgResp},
        openim_sdkws::MsgData,
    },
    rand::{self, Rng},
    tonic::transport::Server,
    tools::mq_producer::kafka::KafkaConfig,
    utils::time_util,
    ErrorKind, Result,
};
use serde::{Deserialize, Serialize};
use openim_storage::controller::msg::{BaseMsgDatabase, MsgDatabase};
use tools::discover::{RegisterCenter, RpcConfig};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MsgConfig {
    pub share: Share,
    pub rpc: RpcConfig,
    pub kafka: KafkaConfig,
    pub topics: MQTopcis,
}

#[derive(Clone)]
pub struct MsgRpcServer {
    config: Arc<MsgConfig>,
    msg_database: Arc<dyn MsgDatabase>,
}

impl MsgRpcServer {
    pub async fn start(config: &MsgConfig, register_center: Box<dyn RegisterCenter>) -> Result<()> {
        let config_arc = Arc::new(config.clone());

        let msg_rpc_server = MsgRpcServer::new(config_arc).await?;

        register_center.register_service(&config.rpc).await?;
        tracing::info!("<open-im-rpc-msg> rpc service register to service register center");

        let msg_server = MsgServer::new(msg_rpc_server);

        tracing::info!(
            "<open-im-rpc-msg> rpc service started at {}",
            config.rpc.rpc_server_url()
        );

        Server::builder()
            .add_service(msg_server)
            .serve(config.rpc.rpc_server_url().parse().unwrap())
            .await?;
        Ok(())
    }

    pub async fn new(config: Arc<MsgConfig>) -> Result<Self> {
        let database =
            BaseMsgDatabase::new_kafka(&config.kafka, &config.topics.to_redis_topic).await?;

        Ok(Self {
            config,
            msg_database: Arc::new(database),
        })
    }
}

impl MsgRpcServer {
    pub async fn modify_message_by_user_message_receive_opt(
        &self,
        _msg: &mut MsgData,
        _user_id: &str,
        _conversation_id: &str,
        _session_type: i32,
    ) -> Result<bool> {
        Ok(true)
    }

    pub fn message_verification(&self, msg_data: &MsgData) -> Result<()> {
        match MsgSessionType::from(msg_data.session_type) {
            MsgSessionType::SingleChatType => {
                if self
                    .config
                    .share
                    .im_admin_user_id
                    .contains(&msg_data.send_id)
                {
                    return Ok(());
                }

                if MsgContentType::is_notification(msg_data.content_type) {
                    return Ok(());
                }

                //todo webhookBeforeSendSingleMsg
                //todo Black
            }
            MsgSessionType::ReadGroupChatType => {}
            _ => {}
        }

        Ok(())
    }

    pub async fn send_msg_single_chat(&self, mut msg_data: MsgData) -> Result<SendMsgResp> {
        self.message_verification(&msg_data)?;

        let mut send = true;

        let recv_id = msg_data.recv_id.clone();
        let send_id = msg_data.recv_id.clone();

        let msg_options = MsgOptions::from_msg_data(&mut msg_data);

        let is_notification = msg_options.is_notification();

        if !is_notification {
            send = self
                .modify_message_by_user_message_receive_opt(
                    &mut msg_data,
                    &recv_id,
                    &conversation_util::gen_conversation_id_for_single(&send_id, &recv_id),
                    constant::SINGLE_CHAT_TYPE,
                )
                .await?;
        }

        if !send {
            todo!()
        } else {
            self.msg_database
                .msg_to_mq(
                    &conversation_util::gen_conversation_unique_key_for_single(&send_id, &recv_id),
                    &msg_data,
                )
                .await?;

            todo!()
        }
    }
    pub async fn send_msg_notification(&self, _msg_data: MsgData) -> Result<SendMsgResp> {
        todo!()
    }
    pub async fn send_msg_group_chat(&self, _msg_data: MsgData) -> Result<SendMsgResp> {
        todo!()
    }

    pub async fn send_msg(&self, req: SendMsgReq) -> Result<SendMsgResp> {
        if req.msg_data.is_none() {
            return Err(ErrorKind::MsgDataIsNil.into());
        }

        let mut msg_data = req.msg_data.unwrap();

        encapsulate_msg_data(&mut msg_data);

        //todo stream contentType

        match MsgSessionType::from(msg_data.session_type) {
            MsgSessionType::SingleChatType => {
                return self.send_msg_single_chat(msg_data).await;
            }
            MsgSessionType::NotificationChatType => {
                return self.send_msg_notification(msg_data).await;
            }
            MsgSessionType::ReadGroupChatType => {
                return self.send_msg_group_chat(msg_data).await;
            }
            _ => {
                return Err(ErrorKind::UnknowedSessionType.into());
            }
        }
    }
}

pub fn encapsulate_msg_data(msg: &mut MsgData) {
    msg.server_msg_id = get_msg_id(&msg.send_id);

    match MsgContentType::from(msg.content_type) {
        MsgContentType::Text
        | MsgContentType::Picture
        | MsgContentType::Voice
        | MsgContentType::Video
        | MsgContentType::File
        | MsgContentType::AtText
        | MsgContentType::Merger
        | MsgContentType::Card
        | MsgContentType::Location
        | MsgContentType::Custom
        | MsgContentType::Quote
        | MsgContentType::Revoke => {
            MsgContentType::on_revoke(&mut msg.options);
        }
        MsgContentType::HasReadReceipt => {
            MsgContentType::on_has_read_receipt(&mut msg.options);
        }
        MsgContentType::Typing => {
            MsgContentType::on_typing(&mut msg.options);
        }
        _ => {}
    }
}

pub fn get_msg_id(send_id: &str) -> String {
    let mut rng = rand::rng();
    let tmp = time_util::get_current_time_formatted() + send_id + &rng.random::<u32>().to_string();
    md5(tmp.as_bytes())
}
