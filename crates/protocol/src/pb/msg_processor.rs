use std::collections::HashMap;

use super::{
    constant::{constant, MsgSessionType},
    openim_sdkws::MsgData,
};

pub struct MsgOptions<'a>(&'a mut HashMap<String, bool>);

impl<'a> MsgOptions<'a> {
    pub fn set_unread_count(&mut self, v: bool) {
        self.0.insert(constant::IS_UNREAD_COUNT.to_string(), v);
    }

    pub fn is_unread_count(&self) -> bool {
        return self
            .0
            .get(constant::IS_UNREAD_COUNT)
            .and_then(|v| Some(*v))
            .unwrap_or_default();
    }

    pub fn set_offline_push(&mut self, v: bool) {
        self.0.insert(constant::IS_OFFLINE_PUSH.to_string(), v);
    }

    pub fn is_offline_push(&self) -> bool {
        return self
            .0
            .get(constant::IS_OFFLINE_PUSH)
            .and_then(|v| Some(*v))
            .unwrap_or_default();
    }

    pub fn is_history(&self) -> bool {
        return self
            .0
            .get(constant::IS_HISTORY)
            .and_then(|v| Some(*v))
            .unwrap_or_default();
    }

    pub fn is_send_msg(&self) -> bool {
        return self
            .0
            .get(constant::IS_SEND_MSG)
            .and_then(|v| Some(*v))
            .unwrap_or_default();
    }

    pub fn is_not_notification(&self) -> bool {
        return self
            .0
            .get(constant::IS_NOT_NOTIFICATION)
            .and_then(|v| Some(*v))
            .unwrap_or_default();
    }

    pub fn is_notification(&self) -> bool {
        return !self.is_not_notification();
    }

    pub fn from_msg_data(msg_data: &'a mut MsgData) -> Self {
        MsgOptions(&mut msg_data.options)
    }
}

pub fn new_msg_options() -> HashMap<String, bool> {
    let mut options = HashMap::with_capacity(11);
    options.insert(constant::IS_OFFLINE_PUSH.to_string(), false);
    options
}

pub fn get_chat_conversation_id_by_msg(msg: &MsgData) -> String {
    let session_type = MsgSessionType::from(msg.session_type);
    session_type.get_chat_conversation_id_by_msg(msg)
}

pub fn get_notification_conversation_id_by_msg(msg: &MsgData) -> String {
    let session_type = MsgSessionType::from(msg.session_type);
    session_type.get_notification_conversation_id_by_msg(msg)
}

pub fn is_group_conversation_id(conversation_id: &str) -> bool {
    return has_prefix(conversation_id, "g_") || has_prefix(conversation_id, "sg_");
}

pub fn has_prefix(key: &str, prefix: &str) -> bool {
    let prefix_len = prefix.len();
    if key.len() < prefix_len {
        return false;
    }

    if key[0..prefix_len].to_owned() == prefix {
        return true;
    } else {
        return false;
    }
}
