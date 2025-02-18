use std::collections::HashMap;

use super::{constant::constant, openim_sdkws::MsgData};

pub fn is_notification_by_msg(msg: &MsgData) -> bool {
    return !is_not_notification(&msg.options);
}

fn is_not_notification(option: &HashMap<String, bool>) -> bool {
    return option
        .get(constant::IS_NOT_NOTIFICATION)
        .and_then(|v| Some(*v))
        .unwrap_or_default();
}
