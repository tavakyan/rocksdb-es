use crate::repository::RocksDbEventRepository;
use crate::types::RocksDbCqrs;
use cqrs_es::persist::PersistedEventStore;
use cqrs_es::{Aggregate, CqrsFramework, Query};
use rocksdb::DB;
use std::sync::{Arc, RwLock};

/// Create a CqrsFramework that uses RocksDb
pub fn rocks_db_cqrs<A>(
    db: Arc<RwLock<DB>>,
    query_processor: Vec<Box<dyn Query<A>>>,
    services: A::Services,
) -> RocksDbCqrs<A>
where
    A: Aggregate,
{
    let repo = RocksDbEventRepository::new(db);
    let store = PersistedEventStore::new_event_store(repo);
    CqrsFramework::new(store, query_processor, services)
}

#[cfg(test)]
mod test {
    use crate::testing::tests::{
        get_rocks_db_storage_path, TestAggregate, TestQueryRepository, TestServices, TestView,
    };
    use crate::{rocks_db_cqrs, RocksDbViewRepository};
    use rocksdb;
    use std::sync::{Arc, RwLock};
    use tempfile::tempfile;

    #[tokio::test]
    async fn test_valid_cqrs_framework() {
        // let pool = default_postgress_pool(TEST_CONNECTION_STRING).await;
        let (tmp_dir, file_path) = get_rocks_db_storage_path();

        // / open a DB with specifying ColumnFamilies
        let db = rocksdb::DB::open_default(file_path).unwrap();
        let db = Arc::new(RwLock::new(db));

        {
            let repo =
                RocksDbViewRepository::<TestView, TestAggregate>::new("test_view", db.clone());
            let query = TestQueryRepository::new(Arc::new(repo));
            let cqrs = rocks_db_cqrs(db, vec![Box::new(query)], TestServices);
        }
    }
}
