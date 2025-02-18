use protocol::tonic::Status;
use serde_json::Error as SerdeJsonError;
use thiserror::Error;
use protocol::prost::EncodeError as ProstEncodeError;
use rdkafka::error::KafkaError;
use protocol::tonic::transport::Error as TonicTransportError;

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
    #[error("prost encode error: {0}")]
    ProstEncodeError(#[from] ProstEncodeError),
    #[error("kafka error: {0}")]
    KafkaError(#[from] KafkaError),
    #[error("tonic transport error: {0}")]
    TonicTransportError(#[from] TonicTransportError),
}

pub type Result<T, E = Error> = std::result::Result<T, E>;

impl From<Error> for Status {
    fn from(_value: Error) -> Self {
        todo!()
    }
}
