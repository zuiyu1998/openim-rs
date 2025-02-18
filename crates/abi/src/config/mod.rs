use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KafkaConfig {
    pub name: Option<String>,
    pub password: Option<String>,
    pub connect_timeout: u16,
    pub timeout: u16,
    pub to_redis_topic: String,
    pub broker: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NacosConfig {
    pub ip: String,
    pub port: u16,
    pub teant: String,
}

impl NacosConfig {
    pub fn endpoint_addrs(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RpcConfig {
    pub host: String,
    pub port: u16,
    pub service_name: String,
    pub group_name: String
}

impl RpcConfig {
    pub fn rpc_server_url(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
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
