use abi::{config::Share, tokio, utils::config_util::load_config_with_file_name, Result};

use rpc::msg::{MsgConfig, MsgRpcServer};
use tools::{discover::{nacos::new_nacos_register_center, NacosConfig, RpcConfig}, mq_producer::kafka::KafkaConfig};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    let kafka: KafkaConfig = load_config_with_file_name("config/kafka.yaml");
    let share: Share = load_config_with_file_name("config/share.yaml");
    let rpc: RpcConfig = load_config_with_file_name("config/openim-rpc-msg.yaml");

    let config = MsgConfig { kafka, share, rpc };

    let nacos: NacosConfig = load_config_with_file_name("config/nacos.yaml");
    let server_center = new_nacos_register_center(&nacos);

    MsgRpcServer::start(&config, server_center).await?;

    Ok(())
}
