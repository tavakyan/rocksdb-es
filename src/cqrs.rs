use crate::repository::RocksDbEventRepository;
use crate::types::RocksDbCqrs;
use cqrs_es::persist::PersistedEventStore;
use cqrs_es::{Aggregate, CqrsFramework, Query};
use rocksdb::DB;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

/// Create a CqrsFramework that uses RocksDb
pub fn rocks_db_cqrs<A>(
    path: PathBuf,
    query_processor: Vec<Box<dyn Query<A>>>,
    services: A::Services,
) -> RocksDbCqrs<A>
where
    A: Aggregate,
{
    let db = rocksdb::DB::open_default(path).unwrap();
    let db = Arc::new(RwLock::new(db));
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

        //
        // let db = rocksdb::DB::open_default(file_path).unwrap();
        // let db = Arc::new(RwLock::new(db));

        {
            let repo = RocksDbViewRepository::<TestView, TestAggregate>::new(
                "test_view",
                file_path.clone(),
            );
            let query = TestQueryRepository::new(Arc::new(repo));
            let cqrs = rocks_db_cqrs(file_path.clone(), vec![Box::new(query)], TestServices);
        }
    }
}
