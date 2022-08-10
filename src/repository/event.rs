use async_trait::async_trait;
use cqrs_es::Aggregate;
use cqrs_es::persist::{PersistedEventRepository, PersistenceError, ReplayStream, SerializedEvent, SerializedSnapshot};

pub struct RocksDbEventRepository;

#[async_trait]
impl PersistedEventRepository for RocksDbEventRepository {
    async fn get_events<A: Aggregate>(&self, aggregate_id: &str) -> Result<Vec<SerializedEvent>, PersistenceError> {
        todo!()
    }

    async fn get_last_events<A: Aggregate>(&self, aggregate_id: &str, last_sequence: usize) -> Result<Vec<SerializedEvent>, PersistenceError> {
        todo!()
    }

    async fn get_snapshot<A: Aggregate>(&self, aggregate_id: &str) -> Result<Option<SerializedSnapshot>, PersistenceError> {
        todo!()
    }

    async fn persist<A: Aggregate>(&self, events: &[SerializedEvent], snapshot_update: Option<(String, serde_json::value::Value, usize)>) -> Result<(), PersistenceError> {
        todo!()
    }

    async fn stream_events<A: Aggregate>(&self, aggregate_id: &str) -> Result<ReplayStream, PersistenceError> {
        todo!()
    }

    async fn stream_all_events<A: Aggregate>(&self) -> Result<ReplayStream, PersistenceError> {
        todo!()
    }
}