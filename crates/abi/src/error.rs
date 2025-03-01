use protocol::tonic::transport::Error as TonicTransportError;
use protocol::tonic::Status;
use serde_json::Error as SerdeJsonError;
use thiserror::Error;
use protocol::prost::DecodeError;
use tools::{discover::DiscoverError, mq_producer::MQError};
use redis::RedisError;
use mongodb::error::Error as MongodbError;

#[derive(Debug, Error)]
pub enum ErrorKind {
    #[error("msgData is nil")]
    MsgDataIsNil,
    #[error("unknown sessionType")]
    UnknowedSessionType,
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("abi error kind: {0}")]
    Kind(#[from] ErrorKind),
    #[error("serde json error: {0}")]
    SerdeJsonError(#[from] SerdeJsonError),
    #[error("tonic transport error: {0}")]
    TonicTransportError(#[from] TonicTransportError),
    #[error("discover error: {0}")]
    DiscoverError(#[from] DiscoverError),
    #[error("mq error: {0}")]
    MQError(#[from] MQError),
    #[error("prost decode error: {0}")]
    DecodeError(#[from] DecodeError),
    #[error("redis error: {0}")]
    RedisError(#[from] RedisError),
    #[error("mongodb error: {0}")]
    MongodbError(#[from] MongodbError),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

impl From<Error> for Status {
    fn from(_value: Error) -> Self {
        todo!()
    }
}
