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
