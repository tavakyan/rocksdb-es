use cqrs_es::persist::PersistenceError;
use thiserror::Error;

/// RocksDbAggregateError
#[derive(Error, Debug)]
pub enum RocksDbAggregateError {
    #[error("Error with optimistic locking")]
    OptimisticLock,
    #[error("Error with deserializing aggregate")]
    DeserializationError,
    #[error("Error opening path file for aggregate persistence")]
    PathError,
    #[error("Unknown error in aggregate.")]
    UnknownError,
}

/// Custom result type that returns a type and a RocksDbAggregateError.
pub type Result<T> = std::result::Result<T, RocksDbAggregateError>;

impl From<serde_json::Error> for RocksDbAggregateError {
    fn from(err: serde_json::Error) -> Self {
        match err.classify() {
            serde_json::error::Category::Data | serde_json::error::Category::Syntax => {
                RocksDbAggregateError::DeserializationError
            }
            serde_json::error::Category::Io | serde_json::error::Category::Eof => {
                RocksDbAggregateError::UnknownError
            }
        }
    }
}

impl From<RocksDbAggregateError> for PersistenceError {
    fn from(err: RocksDbAggregateError) -> Self {
        match err {
            RocksDbAggregateError::OptimisticLock => PersistenceError::OptimisticLockError,
            RocksDbAggregateError::PathError => PersistenceError::ConnectionError(Box::new(err)),
            RocksDbAggregateError::DeserializationError => {
                PersistenceError::UnknownError(Box::new(err))
            }
            RocksDbAggregateError::UnknownError => PersistenceError::UnknownError(Box::new(err)),
        }
    }
}
