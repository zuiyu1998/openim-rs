use std::sync::Arc;

use abi::{
    config::{NacosConfig, RpcConfig},
    tonic::{async_trait, transport::Channel},
    Error, Result,
};
use nacos_rust_client::client::{
    naming_client::{Instance, ServiceInstanceKey},
    ClientBuilder, NamingClient,
};
use nacos_tonic_discover::TonicDiscoverFactory;

use super::RegisterCenter;

pub struct NacosRegisterCenter(Arc<NamingClient>);

pub fn new_nacos_register_center(config: &NacosConfig) -> Box<dyn RegisterCenter> {
    Box::new(NacosRegisterCenter::init(config))
}

impl NacosRegisterCenter {
    pub fn init(config: &NacosConfig) -> Self {
        let naming_client = ClientBuilder::new()
            .set_endpoint_addrs(&config.endpoint_addrs())
            .set_tenant(config.teant.to_owned())
            .set_use_grpc(true) //select communication protocol
            .build_naming_client();

        Self(naming_client)
    }
}

#[async_trait]
impl RegisterCenter for NacosRegisterCenter {
    //注册rpc服务
    async fn register_service(&self, config: &RpcConfig) -> Result<()> {
        let instance = Instance::new_simple(
            &config.host,
            config.port as u32,
            &config.service_name,
            &config.group_name,
        );

        self.0.register(instance);

        Ok(())
    }

    //获取tonic 客户端
    async fn get_grpc_clint(&self, config: &RpcConfig) -> Result<Channel> {
        let discover_factory = TonicDiscoverFactory::new(self.0.clone());

        let service_key = ServiceInstanceKey::new(&config.service_name, &config.group_name);

        let channel = discover_factory
            .build_service_channel(service_key.clone())
            .await
            .map_err(|e| Error::NacosError(e.to_string()))?;

        Ok(channel)
    }
}
