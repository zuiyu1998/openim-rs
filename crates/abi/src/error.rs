use protocol::tonic::transport::Error as TonicTransportError;
use protocol::tonic::Status;
use serde_json::Error as SerdeJsonError;
use thiserror::Error;

use tools::{discover::DiscoverError, mq_producer::MQError};

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
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

impl From<Error> for Status {
    fn from(_value: Error) -> Self {
        todo!()
    }
}
