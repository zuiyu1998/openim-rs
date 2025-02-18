use std::collections::HashMap;

pub mod constant {

    pub const NOTIFICATION_BEGIN: i32 = 1000;
    pub const NOTIFICATION_END: i32 = 5000;

    pub const IS_UNREAD_COUNT: &str = "unreadCount";
    pub const IS_OFFLINE_PUSH: &str = "offlinePush";
    pub const IS_CONVERSATION_UPDATETE: &str = "conversationUpdate";
    pub const IS_SENDER_CONVERSATION_UPDATE: &str = "senderConversationUpdate";
    pub const IS_SENDER_SYNCNCNC: &str = "senderSync";
    pub const IS_PERSISTENT: &str = "persistent";
    pub const IS_HISTORY: &str = "history";
    pub const IS_NOT_NOTIFICATION: &str = "isNotNotification";

    pub const SINGLE_CHAT_TYPE: i32 = 1;
}

pub enum MsgSessionType {
    SingleChatType,
    NotificationChatType,
    ReadGroupChatType,
    Unknowed,
}

impl From<i32> for MsgSessionType {
    fn from(value: i32) -> Self {
        match value {
            1 => MsgSessionType::SingleChatType,
            4 => MsgSessionType::NotificationChatType,
            3 => MsgSessionType::ReadGroupChatType,
            _ => MsgSessionType::Unknowed,
        }
    }
}

pub enum MsgContentType {
    Text,
    Picture,
    Voice,
    Video,
    File,
    AtText,
    Merger,
    Card,
    Location,
    Custom,
    Quote,
    Revoke,
    HasReadReceipt,
    Typing,
    Unknowed,
}

impl MsgContentType {
    pub fn is_notification(content_type: i32) -> bool {
        if content_type <= constant::NOTIFICATION_END
            && content_type >= constant::NOTIFICATION_BEGIN
        {
            return true;
        } else {
            return false;
        }
    }

    pub fn on_revoke(options: &mut HashMap<String, bool>) {
        options.insert(constant::IS_UNREAD_COUNT.to_string(), false);
        options.insert(constant::IS_OFFLINE_PUSH.to_string(), false);
    }

    pub fn on_has_read_receipt(options: &mut HashMap<String, bool>) {
        options.insert(constant::IS_UNREAD_COUNT.to_string(), false);
        options.insert(constant::IS_OFFLINE_PUSH.to_string(), false);
        options.insert(constant::IS_CONVERSATION_UPDATETE.to_string(), false);
        options.insert(constant::IS_SENDER_CONVERSATION_UPDATE.to_string(), false);
    }

    pub fn on_typing(options: &mut HashMap<String, bool>) {
        options.insert(constant::IS_UNREAD_COUNT.to_string(), false);
        options.insert(constant::IS_OFFLINE_PUSH.to_string(), false);
        options.insert(constant::IS_CONVERSATION_UPDATETE.to_string(), false);
        options.insert(constant::IS_SENDER_CONVERSATION_UPDATE.to_string(), false);
        options.insert(constant::IS_HISTORY.to_string(), false);
        options.insert(constant::IS_PERSISTENT.to_string(), false);
        options.insert(constant::IS_SENDER_SYNCNCNC.to_string(), false);
    }
}

impl From<i32> for MsgContentType {
    fn from(value: i32) -> Self {
        match value {
            101 => MsgContentType::Text,
            102 => MsgContentType::Picture,
            103 => MsgContentType::Voice,
            104 => MsgContentType::Video,
            105 => MsgContentType::File,
            106 => MsgContentType::AtText,
            107 => MsgContentType::Merger,
            108 => MsgContentType::Card,
            109 => MsgContentType::Location,
            110 => MsgContentType::Custom,
            114 => MsgContentType::Quote,
            111 => MsgContentType::Revoke,
            2200 => MsgContentType::HasReadReceipt,
            113 => MsgContentType::Typing,
            _ => MsgContentType::Unknowed,
        }
    }
}
