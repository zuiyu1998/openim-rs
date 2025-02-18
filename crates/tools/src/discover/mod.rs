pub mod nacos;

use abi::{config::RpcConfig, tonic::{async_trait, transport::Channel}, Result};


#[async_trait]
pub trait RegisterCenter {
    //注册rpc服务
    async fn register_service(&self, config: &RpcConfig) -> Result<()>;

    //获取tonic 客户端
    async fn get_grpc_clint(&self, config: &RpcConfig) -> Result<Channel>;
}
