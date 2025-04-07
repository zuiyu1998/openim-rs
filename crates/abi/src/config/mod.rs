use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MQTopcis {
    pub to_redis_topic: String,
    pub to_redis_topic_group_id: String,
    pub to_mongo_topic: String,
    pub to_mongo_topic_group_id: String,
    pub to_push_topic: String,
    pub to_push_topic_group_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Share {
    pub secret: String,
    pub im_admin_user_id: Vec<String>,
    pub multi_login: MultiLogin,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MultiLogin {
    pub policy: isize,
    pub max_num_one_end: isize,
}
