use crate::repository::RocksDbEventRepository;
use crate::types::RocksDbCqrs;
use cqrs_es::persist::PersistedEventStore;
use cqrs_es::{Aggregate, CqrsFramework, Query};

/// Create a CqrsFramework that uses RocksDb
pub fn rocks_db_cqrs<A>(
    query_processor: Vec<Box<dyn Query<A>>>,
    services: A::Services,
) -> RocksDbCqrs<A>
where
    A: Aggregate,
{
    let repo = RocksDbEventRepository {};
    let store = PersistedEventStore::new_event_store(repo);
    CqrsFramework::new(store, query_processor, services)
}

// TODO: Port test_valid_cqrs_framework, but first port test aggregate
