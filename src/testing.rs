#[cfg(test)]
pub(crate) mod tests {
    use crate::RocksDbViewRepository;
    use async_trait::async_trait;
    use cqrs_es::persist::{GenericQuery, SerializedEvent, SerializedSnapshot};
    use cqrs_es::{Aggregate, DomainEvent, EventEnvelope, View};
    use serde::{Deserialize, Serialize};
    use serde_json::Value;
    use std::fmt::{Display, Formatter};
    use std::fs::File;
    use tempfile::tempdir;

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub(crate) struct TestAggregate {
        pub(crate) id: String,
        pub(crate) description: String,
        pub(crate) tests: Vec<String>,
    }

    #[async_trait]
    impl Aggregate for TestAggregate {
        type Command = TestCommand;
        type Event = TestEvent;
        type Error = TestError;
        type Services = TestServices;

        fn aggregate_type() -> String {
            "TestAggregate".to_string()
        }

        async fn handle(
            &self,
            _command: Self::Command,
            _services: &Self::Services,
        ) -> Result<Vec<Self::Event>, Self::Error> {
            Ok(vec![])
        }

        fn apply(&mut self, _e: Self::Event) {}
    }

    impl Default for TestAggregate {
        fn default() -> Self {
            TestAggregate {
                id: "".to_string(),
                description: "".to_string(),
                tests: Vec::new(),
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
    pub(crate) enum TestEvent {
        Created(Created),
        Tested(Tested),
        SomethingElse(SomethingElse),
    }

    #[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
    pub(crate) struct Created {
        pub id: String,
    }

    #[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
    pub(crate) struct Tested {
        pub test_name: String,
    }

    #[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
    pub struct SomethingElse {
        pub description: String,
    }

    impl DomainEvent for TestEvent {
        fn event_type(&self) -> String {
            match self {
                TestEvent::Created(_) => "Created".to_string(),
                TestEvent::Tested(_) => "Tested".to_string(),
                TestEvent::SomethingElse(_) => "SomethingElse".to_string(),
            }
        }

        fn event_version(&self) -> String {
            "1.0".to_string()
        }
    }

    #[derive(Debug, PartialEq)]
    pub(crate) struct TestError(String);

    #[derive(Debug)]
    pub(crate) struct TestServices;

    impl Display for TestError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl std::error::Error for TestError {}

    pub(crate) enum TestCommand {}

    pub(crate) type TestQueryRepository =
        GenericQuery<RocksDbViewRepository<TestView, TestAggregate>, TestView, TestAggregate>;

    #[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
    pub(crate) struct TestView {
        pub(crate) events: Vec<TestEvent>,
    }

    impl View<TestAggregate> for TestView {
        fn update(&mut self, event: &EventEnvelope<TestAggregate>) {
            self.events.push(event.payload.clone());
        }
    }

    pub(crate) const TEST_STORAGE_FILE_NAME: &'static str = "TEMPORARY_ROCKS_DB_TEST_STORAGE";

    pub(crate) fn get_rocks_db_storage_path() -> File {
        let tmp = tempdir().expect("Error: Unable to create a temporary directory");

        let file_path = tmp.path().join(TEST_STORAGE_FILE_NAME);
        let mut file = File::create(file_path).expect("Error: Unable to create a temporary file");
        file
    }
    pub(crate) fn test_event_envelope(
        id: &str,
        sequence: usize,
        event: TestEvent,
    ) -> SerializedEvent {
        let payload: Value = serde_json::to_value(&event).unwrap();
        SerializedEvent {
            aggregate_id: id.to_string(),
            sequence,
            aggregate_type: TestAggregate::aggregate_type().to_string(),
            event_type: event.event_type().to_string(),
            event_version: event.event_version().to_string(),
            payload,
            metadata: Default::default(),
        }
    }

    pub(crate) fn snapshot_context(
        aggregate_id: String,
        current_sequence: usize,
        current_snapshot: usize,
        aggregate: Value,
    ) -> SerializedSnapshot {
        SerializedSnapshot {
            aggregate_id,
            aggregate,
            current_sequence,
            current_snapshot,
        }
    }

}
