use cqrs_es::AggregateError;
use rocksdb::{Error as RocksError, Error, ErrorKind};
use std::fmt::{Display, Formatter};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RocksDbAggregateError {
    EventStoreError,
    RocksDbError,
    UnknownError,
}

impl Display for RocksDbAggregateError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl From<rocksdb::Error> for RocksDbAggregateError {
    fn from(err: RocksError) -> Self {
        match err.kind() {
            ErrorKind::NotFound => {}
            ErrorKind::Corruption => {}
            ErrorKind::NotSupported => {}
            ErrorKind::InvalidArgument => {}
            ErrorKind::IOError => {}
            ErrorKind::MergeInProgress => {}
            ErrorKind::Incomplete => {}
            ErrorKind::ShutdownInProgress => {}
            ErrorKind::TimedOut => {}
            ErrorKind::Aborted => {}
            ErrorKind::Busy => {}
            ErrorKind::Expired => {}
            ErrorKind::TryAgain => {}
            ErrorKind::CompactionTooLarge => {}
            ErrorKind::ColumnFamilyDropped => {}
            ErrorKind::Unknown => {}
        }
        todo!()
    }
}

impl<T: std::error::Error> From<RocksDbAggregateError> for AggregateError<T> {
    fn from(err: RocksDbAggregateError) -> Self {
        match err {
            RocksDbAggregateError::EventStoreError => {}
            RocksDbAggregateError::RocksDbError => {}
            RocksDbAggregateError::UnknownError => {}
        }
        todo!()
    }
}

impl From<serde_json::Error> for RocksDbAggregateError {
    fn from(err: serde_json::Error) -> Self {
        todo!("")
    }
}

use rocksdb::{Options, DB};

fn test() {
    let path = "_path_for_rocksdb_storage";
    {
        let db = DB::open_default(path);
        db.put(b"my key", b"my value").unwrap();
        match db.get(b"my key") {
            Ok(Some(value)) => println!("retrieved value {}", String::from_utf8(value).unwrap()),
            Ok(None) => println!("value not found"),
            Err(e) => println!("operational problem encountered: {}", e),
        }
        db.delete(b"my key").unwrap();
    }
    let _ = DB::destroy(&Options::default(), path);
}
