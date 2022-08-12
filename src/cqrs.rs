use crate::repository::RocksDbEventRepository;
use crate::types::RocksDbCqrs;
use cqrs_es::persist::PersistedEventStore;
use cqrs_es::{Aggregate, CqrsFramework, Query};

/// Create a CqrsFramework that uses RocksDb
pub fn rocks_db_cqrs<A>(
    path: &str,
    query_processor: Vec<Box<dyn Query<A>>>,
    services: A::Services,
) -> RocksDbCqrs<A>
where
    A: Aggregate,
{
    let repo = RocksDbEventRepository::new(path);
    let store = PersistedEventStore::new_event_store(repo);
    CqrsFramework::new(store, query_processor, services)
}

#[cfg(test)]
mod test {
    use crate::testing::tests::{
        get_rocks_db_storage_path, TestAggregate, TestQueryRepository, TestServices, TestView,
    };
    use crate::{rocks_db_cqrs, RocksDbViewRepository};
    use std::sync::Arc;

    #[tokio::test]
    async fn test_valid_cqrs_framework() {
        // let pool = default_postgress_pool(TEST_CONNECTION_STRING).await;
        let (tmp_dir, file_path) = get_rocks_db_storage_path();
        let path = file_path
            .as_os_str()
            .to_str()
            .expect("failed to get string from os string");
        let repo = RocksDbViewRepository::<TestView, TestAggregate>::new("test_view", path);
        let query = TestQueryRepository::new(Arc::new(repo));
        let _ps = rocks_db_cqrs(path, vec![Box::new(query)], TestServices);
    }
}
