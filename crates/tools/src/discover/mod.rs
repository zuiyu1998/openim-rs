pub mod nacos;

use serde::{Deserialize, Serialize};
use thiserror::Error;
use tonic::{async_trait, transport::Channel};

#[derive(Debug, Error)]
pub enum DiscoverError {
    #[error("nacos error: {0}")]
    NacosError(String),
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
    pub group_name: String,
}

impl RpcConfig {
    pub fn rpc_server_url(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}

#[async_trait]
pub trait RegisterCenter {
    //注册rpc服务
    async fn register_service(&self, config: &RpcConfig) -> Result<(), DiscoverError>;

    //获取tonic 客户端
    async fn get_grpc_clint(&self, config: &RpcConfig) -> Result<Channel, DiscoverError>;
}
