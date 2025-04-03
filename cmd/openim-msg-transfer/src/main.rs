use abi::{Result, config::MQTopcis, tokio, utils::config_util::load_config_with_file_name};

use openim_storage::{cache::redis::RedisConfig, database::mongodb::MongodbConfig};
use tools::{batcher::BatcherConfig, mq_producer::kafka::KafkaConfig};

use msg_transfer::{MsgTransferSevice, MsgTransferSeviceConfig};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();

    let kafka: KafkaConfig = load_config_with_file_name("config/kafka.yaml");
    let topics: MQTopcis = load_config_with_file_name("config/topics.yaml");
    let redis: RedisConfig = load_config_with_file_name("config/redis.yaml");
    let mongo: MongodbConfig = load_config_with_file_name("config/mongo.yaml");
    let batcher: BatcherConfig = load_config_with_file_name("config/batcher.yaml");

    let config = MsgTransferSeviceConfig {
        kafka,
        topics,
        redis,
        mongo,
        batcher
    };

    MsgTransferSevice::start(&config).await?;

    Ok(())
}
