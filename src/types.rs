use crate::repository::RocksDbEventRepository;
use cqrs_es::persist::PersistedEventStore;
use cqrs_es::CqrsFramework;
/// A convenience type for a CqrsFramework backed by a RocksDb event repository
pub type RocksDbCqrs<A> = CqrsFramework<A, PersistedEventStore<RocksDbEventRepository, A>>;
