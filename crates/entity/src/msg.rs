use std::collections::HashMap;

use abi::protocol::pb::openim_sdkws::MsgData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OfflinePushModel {
    pub title: String,
    pub desc: String,
    pub ex: String,
    pub ios_push_sound: String,
    pub ios_badge_count: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MsgDocModel {
    pub doc_id: String,
    pub msgs: Vec<MsgInfoModel>,
}

impl MsgDocModel {
    const SINGLE_GOC_MSG_NUM: i64 = 100;

    pub fn table_name() -> &'static str {
        "msg"
    }

    pub fn get_single_goc_msg_num() -> i64 {
        Self::SINGLE_GOC_MSG_NUM
    }

    pub fn get_doc_id(conversation_id: &str, seq: i64) -> String {
        let seq_suffix = (seq - 1) / Self::get_single_goc_msg_num();

        Self::index_gen(conversation_id, seq_suffix)
    }

    pub fn get_msg_index(seq: i64) -> i64 {
        (seq - 1) / Self::get_single_goc_msg_num()
    }

    pub fn index_gen(conversation_id: &str, seq_suffix: i64) -> String {
        format!("{}:{}", conversation_id, seq_suffix)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MsgDataModel {
    pub send_id: String,
    pub recv_id: String,
    pub group_id: String,
    pub client_msg_id: String,
    pub server_msg_id: String,
    pub sender_platform_id: i32,
    pub sender_nickname: String,
    pub sender_face_url: String,
    pub session_type: i32,
    pub msg_from: i32,
    pub content_type: i32,
    pub content: String,
    pub seq: i64,
    pub send_time: i64,
    pub create_time: i64,
    pub status: i32,
    pub is_read: bool,
    pub options: HashMap<String, bool>,
    pub offline_push: Option<OfflinePushModel>,
    pub at_user_id_list: Vec<String>,
    pub attached_info: String,
    pub ex: String,
}

impl MsgDataModel {
    pub fn from_msg_data(msg_data: &MsgData) -> Self {
        MsgDataModel {
            send_id: msg_data.send_id.clone(),
            recv_id: msg_data.recv_id.clone(),
            group_id: msg_data.group_id.clone(),
            client_msg_id: msg_data.client_msg_id.clone(),
            server_msg_id: msg_data.server_msg_id.clone(),
            sender_platform_id: msg_data.sender_platform_id,
            sender_nickname: msg_data.sender_nickname.clone(),
            sender_face_url: msg_data.sender_face_url.clone(),
            session_type: msg_data.session_type,
            msg_from: msg_data.msg_from,
            content_type: msg_data.content_type,
            content: String::from_utf8_lossy(&msg_data.content).to_string(),
            seq: msg_data.seq,
            send_time: msg_data.send_time,
            create_time: msg_data.create_time,
            status: msg_data.status,
            is_read: msg_data.is_read,
            options: msg_data.options.clone(),
            offline_push: msg_data
                .offline_push_info
                .as_ref()
                .map(|info| OfflinePushModel {
                    title: info.title.clone(),
                    desc: info.desc.clone(),
                    ex: info.ex.clone(),
                    ios_push_sound: info.i_os_push_sound.clone(),
                    ios_badge_count: info.i_os_badge_count,
                }),
            at_user_id_list: msg_data.at_user_id_list.clone(),
            attached_info: msg_data.attached_info.clone(),
            ex: msg_data.ex.clone(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RevokeModel {}

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
pub struct MsgInfoModel {
    pub msg: MsgDataModel,
    pub revoke: Option<RevokeModel>,
    pub del_list: Vec<String>,
    pub is_read: bool,
}

impl MsgInfoModel {
    pub fn from_msg_data_model(msg: MsgDataModel) -> Self {
        MsgInfoModel {
            msg,
            revoke: None,
            del_list: vec![],
            is_read: false,
        }
    }

    pub fn from_msg_data(msg_data: &MsgData) -> Self {
        MsgInfoModel::from_msg_data_model(MsgDataModel::from_msg_data(msg_data))
    }
}
