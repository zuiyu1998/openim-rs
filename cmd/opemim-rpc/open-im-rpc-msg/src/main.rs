use std::sync::Arc;

use abi::{
    config::{KafkaConfig, RpcConfig, Share},
    protocol::pb::openim_msg::msg_server::MsgServer,
    tokio,
    tonic::transport::Server,
    utils::config_util::load_config_with_file_name,
    Result,
};

use rpc::msg::{MsgConfig, MsgRpcServer};

#[tokio::main]
async fn main() -> Result<()> {
    let kafka: KafkaConfig = load_config_with_file_name("config/kafka.yaml");
    let share: Share = load_config_with_file_name("config/share.yaml");
    let rpc: RpcConfig = load_config_with_file_name("config/openim-rpc-msg.yaml");

    let config = Arc::new(MsgConfig { kafka, share, rpc });

    let msg_rpc_server = MsgRpcServer::start(config.clone()).await?;

    let msg_server = MsgServer::new(msg_rpc_server);

    Server::builder()
        .add_service(msg_server)
        .serve(config.rpc.rpc_server_url().parse().unwrap())
        .await?;

    Ok(())
}
