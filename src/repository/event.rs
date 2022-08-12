use async_trait::async_trait;
use cqrs_es::persist::{
    PersistedEventRepository, PersistenceError, ReplayStream, SerializedEvent, SerializedSnapshot,
};
use cqrs_es::Aggregate;
use rocksdb::DB;

/// An event repository relying on RocksDB for persistence.
pub struct RocksDbEventRepository {
    db: DB,
}

impl RocksDbEventRepository {
    pub fn new(path: &str) -> Self {
        let db = DB::open_default(path).unwrap();
        Self { db }
    }
}

#[async_trait]
impl PersistedEventRepository for RocksDbEventRepository {
    async fn get_events<A: Aggregate>(
        &self,
        aggregate_id: &str,
    ) -> Result<Vec<SerializedEvent>, PersistenceError> {
        todo!()
    }

    async fn get_last_events<A: Aggregate>(
        &self,
        aggregate_id: &str,
        last_sequence: usize,
    ) -> Result<Vec<SerializedEvent>, PersistenceError> {
        todo!()
    }

    async fn get_snapshot<A: Aggregate>(
        &self,
        aggregate_id: &str,
    ) -> Result<Option<SerializedSnapshot>, PersistenceError> {
        todo!()
    }

    async fn persist<A: Aggregate>(
        &self,
        events: &[SerializedEvent],
        snapshot_update: Option<(String, serde_json::value::Value, usize)>,
    ) -> Result<(), PersistenceError> {
        todo!()
    }

    async fn stream_events<A: Aggregate>(
        &self,
        aggregate_id: &str,
    ) -> Result<ReplayStream, PersistenceError> {
        todo!()
    }

    async fn stream_all_events<A: Aggregate>(&self) -> Result<ReplayStream, PersistenceError> {
        todo!()
    }
}

#[cfg(test)]
mod test {
    use cqrs_es::persist::PersistedEventRepository;

    use crate::error::RocksDbAggregateError;
    use crate::testing::tests::{
        get_rocks_db_storage_path, snapshot_context, test_event_envelope, Created, SomethingElse,
        TestAggregate, TestEvent, Tested, TEST_STORAGE_FILE_NAME,
    };
    use crate::RocksDbEventRepository;

    #[tokio::test]
    async fn event_repositories() {
        let (tmp, path) = get_rocks_db_storage_path();
        let path = path.to_str().unwrap();
        //     let pool = default_postgress_pool(TEST_CONNECTION_STRING).await;
        let id = uuid::Uuid::new_v4().to_string();
        let event_repo: RocksDbEventRepository = RocksDbEventRepository::new(path);
        //     let events = event_repo.get_events::<TestAggregate>(&id).await.unwrap();
        //     assert!(events.is_empty());
        //
        //     event_repo
        //         .insert_events::<TestAggregate>(&[
        //             test_event_envelope(&id, 1, TestEvent::Created(Created { id: id.clone() })),
        //             test_event_envelope(
        //                 &id,
        //                 2,
        //                 TestEvent::Tested(Tested {
        //                     test_name: "a test was run".to_string(),
        //                 }),
        //             ),
        //         ])
        //         .await
        //         .unwrap();
        //     let events = event_repo.get_events::<TestAggregate>(&id).await.unwrap();
        //     assert_eq!(2, events.len());
        //     events.iter().for_each(|e| assert_eq!(&id, &e.aggregate_id));
        //
        //     // Optimistic lock error
        //     let result = event_repo
        //         .insert_events::<TestAggregate>(&[
        //             test_event_envelope(
        //                 &id,
        //                 3,
        //                 TestEvent::SomethingElse(SomethingElse {
        //                     description: "this should not persist".to_string(),
        //                 }),
        //             ),
        //             test_event_envelope(
        //                 &id,
        //                 2,
        //                 TestEvent::SomethingElse(SomethingElse {
        //                     description: "bad sequence number".to_string(),
        //                 }),
        //             ),
        //         ])
        //         .await
        //         .unwrap_err();
        //     match result {
        //         PostgresAggregateError::OptimisticLock => {}
        //         _ => panic!("invalid error result found during insert: {}", result),
        //     };
        //
        //     let events = event_repo.get_events::<TestAggregate>(&id).await.unwrap();
        //     assert_eq!(2, events.len());
        //
        //     verify_replay_stream(&id, event_repo).await;
    }
    //
    // async fn verify_replay_stream(id: &str, event_repo: PostgresEventRepository) {
    //     let mut stream = event_repo
    //         .stream_events::<TestAggregate>(&id)
    //         .await
    //         .unwrap();
    //     let mut found_in_stream = 0;
    //     while let Some(_) = stream.next::<TestAggregate>().await {
    //         found_in_stream += 1;
    //     }
    //     assert_eq!(found_in_stream, 2);
    //
    //     let mut stream = event_repo
    //         .stream_all_events::<TestAggregate>()
    //         .await
    //         .unwrap();
    //     let mut found_in_stream = 0;
    //     while let Some(_) = stream.next::<TestAggregate>().await {
    //         found_in_stream += 1;
    //     }
    //     assert!(found_in_stream >= 2);
    // }
    //
    // #[tokio::test]
    // async fn snapshot_repositories() {
    //     let pool = default_postgress_pool(TEST_CONNECTION_STRING).await;
    //     let id = uuid::Uuid::new_v4().to_string();
    //     let event_repo: PostgresEventRepository = PostgresEventRepository::new(pool.clone());
    //     let snapshot = event_repo.get_snapshot::<TestAggregate>(&id).await.unwrap();
    //     assert_eq!(None, snapshot);
    //
    //     let test_description = "some test snapshot here".to_string();
    //     let test_tests = vec!["testA".to_string(), "testB".to_string()];
    //     event_repo
    //         .insert::<TestAggregate>(
    //             serde_json::to_value(TestAggregate {
    //                 id: id.clone(),
    //                 description: test_description.clone(),
    //                 tests: test_tests.clone(),
    //             })
    //                 .unwrap(),
    //             id.clone(),
    //             1,
    //             &vec![],
    //         )
    //         .await
    //         .unwrap();
    //
    //     let snapshot = event_repo.get_snapshot::<TestAggregate>(&id).await.unwrap();
    //     assert_eq!(
    //         Some(snapshot_context(
    //             id.clone(),
    //             0,
    //             1,
    //             serde_json::to_value(TestAggregate {
    //                 id: id.clone(),
    //                 description: test_description.clone(),
    //                 tests: test_tests.clone(),
    //             })
    //                 .unwrap()
    //         )),
    //         snapshot
    //     );
    //
    //     // sequence iterated, does update
    //     event_repo
    //         .update::<TestAggregate>(
    //             serde_json::to_value(TestAggregate {
    //                 id: id.clone(),
    //                 description: "a test description that should be saved".to_string(),
    //                 tests: test_tests.clone(),
    //             })
    //                 .unwrap(),
    //             id.clone(),
    //             2,
    //             &vec![],
    //         )
    //         .await
    //         .unwrap();
    //
    //     let snapshot = event_repo.get_snapshot::<TestAggregate>(&id).await.unwrap();
    //     assert_eq!(
    //         Some(snapshot_context(
    //             id.clone(),
    //             0,
    //             2,
    //             serde_json::to_value(TestAggregate {
    //                 id: id.clone(),
    //                 description: "a test description that should be saved".to_string(),
    //                 tests: test_tests.clone(),
    //             })
    //                 .unwrap()
    //         )),
    //         snapshot
    //     );
    //
    //     // sequence out of order or not iterated, does not update
    //     let result = event_repo
    //         .update::<TestAggregate>(
    //             serde_json::to_value(TestAggregate {
    //                 id: id.clone(),
    //                 description: "a test description that should not be saved".to_string(),
    //                 tests: test_tests.clone(),
    //             })
    //                 .unwrap(),
    //             id.clone(),
    //             2,
    //             &vec![],
    //         )
    //         .await
    //         .unwrap_err();
    //     match result {
    //         PostgresAggregateError::OptimisticLock => {}
    //         _ => panic!("invalid error result found during insert: {}", result),
    //     };
    //
    //     let snapshot = event_repo.get_snapshot::<TestAggregate>(&id).await.unwrap();
    //     assert_eq!(
    //         Some(snapshot_context(
    //             id.clone(),
    //             0,
    //             2,
    //             serde_json::to_value(TestAggregate {
    //                 id: id.clone(),
    //                 description: "a test description that should be saved".to_string(),
    //                 tests: test_tests.clone(),
    //             })
    //                 .unwrap()
    //         )),
    //         snapshot
    //     );
    // }
}
