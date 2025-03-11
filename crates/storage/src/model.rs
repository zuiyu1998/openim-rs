use abi::protocol::pb::openim_sdkws::MsgData;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct MsgDataModel {
    pub seq: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RevokeModel {}

#[derive(Debug, Serialize, Deserialize)]
pub struct MsgInfoModel {
    pub msg: MsgDataModel,
    pub revoke: Option<RevokeModel>,
    pub del_list: Vec<String>,
    pub is_read: bool,
}

impl MsgInfoModel {
    pub fn from_msg_data(msg_data: &MsgData) -> Self {
        MsgInfoModel {
            msg: MsgDataModel { seq: msg_data.seq },
            revoke: None,
            del_list: vec![],
            is_read: false,
        }
    }
}
