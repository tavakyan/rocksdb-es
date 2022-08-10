#[cfg(test)]
pub(crate) mod tests {
    use crate::RocksDbViewRepository;
    use async_trait::async_trait;
    use cqrs_es::persist::{GenericQuery, SerializedEvent, SerializedSnapshot};
    use cqrs_es::{Aggregate, DomainEvent, EventEnvelope, View};
    use serde::{Serialize, Deserialize};
    use serde_json::Value;
    use std::fmt::{Display, Formatter};

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
}
