use crate::repository::RocksDbEventRepository;
use cqrs_es::persist::PersistedEventStore;
use cqrs_es::CqrsFramework;

pub type RocksDbCqrs<A> = CqrsFramework<A, PersistedEventStore<RocksDbEventRepository, A>>;
