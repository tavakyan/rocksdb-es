// #[cfg(test)]
// pub(crate) mod tests {
//     use std::error::Error;
//     use crate::RocksDbViewRepository;
//     use async_trait::async_trait;
//     use cqrs_es::persist::{GenericQuery, SerializedEvent, SerializedSnapshot};
//     use cqrs_es::{Aggregate, DomainEvent, EventEnvelope, View};
//     use serde::{Deserialize, Serialize};
//     use serde_json::Value;
//     use std::fmt::{Display, Formatter};
//
//     #[derive(Debug, Serialize, Deserialize, PartialEq)]
//     pub(crate) struct TestAggregate {
//         pub(crate) id: String,
//         pub(crate) description: String,
//         pub(crate) tests: Vec<String>,
//     }
//
//     impl Aggregate for TestAggregate {
//         type Command = ();
//         type Event = ();
//         type Error = ();
//         type Services = ();
//
//         fn aggregate_type() -> String {
//             todo!()
//         }
//
//         async fn handle(&self, command: Self::Command, service: &Self::Services) -> Result<Vec<Self::Event>, Self::Error> {
//             todo!()
//         }
//
//         fn apply(&mut self, event: Self::Event) {
//             todo!()
//         }
//     }
//
//     impl Default for TestAggregate {
//         fn default() -> Self {
//             todo!()
//         }
//     }
//
//     #[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
//     pub(crate) enum TestEvent {
//         Created(Created),
//         Tested(Tested),
//         SomethingElse(SomethingElse),
//     }
//
//     #[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
//     pub(crate) struct Created {
//         pub id: String,
//     }
//
//     #[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
//     pub(crate) struct Tested {
//         pub test_name: String,
//     }
//
//     #[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
//     pub struct SomethingElse {
//         pub description: String,
//     }
//
//     impl DomainEvent for TestEvent {
//         fn event_type(&self) -> String {
//             match self {
//                 TestEvent::Created(_) => "Created".to_string(),
//                 TestEvent::Tested(_) => "Tested".to_string(),
//                 TestEvent::SomethingElse(_) => "SomethingElse".to_string(),
//             }
//         }
//
//         fn event_version(&self) -> String {
//             "1.0".to_string()
//         }
//     }
//
//     #[derive(Debug, PartialEq)]
//     pub(crate) struct TestError(String);
//
//     #[derive(Debug)]
//     pub(crate) struct TestServices;
//
//     impl Display for TestError {
//         fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//             write!(f, "{}", self.0)
//         }
//     }
//
//     pub(crate) enum TestCommand {}
//
//     pub(crate) type TestQueryRepository =
//     GenericQuery<PostgresViewRepository<TestView, TestAggregate>, TestView, TestAggregate>;
//
//     #[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq)]
//     pub(crate) struct TestView {
//         pub(crate) events: Vec<TestEvent>,
//     }
//
//     impl View<TestAggregate> for TestView {
//         fn update(&mut self, event: &EventEnvelope<TestAggregate>) {
//             self.events.push(event.payload.clone());
//         }
//     }
//
//     pub(crate) const TEST_CONNECTION_STRING: &str =
//         "postgresql://test_user:test_pass@127.0.0.1:5432/test";
//
//     pub(crate) fn test_event_envelope(
//         id: &str,
//         sequence: usize,
//         event: TestEvent,
//     ) -> SerializedEvent {
//         let payload: Value = serde_json::to_value(&event).unwrap();
//         SerializedEvent {
//             aggregate_id: id.to_string(),
//             sequence,
//             aggregate_type: TestAggregate::aggregate_type().to_string(),
//             event_type: event.event_type().to_string(),
//             event_version: event.event_version().to_string(),
//             payload,
//             metadata: Default::default(),
//         }
//     }
//
//     pub(crate) fn snapshot_context(
//         aggregate_id: String,
//         current_sequence: usize,
//         current_snapshot: usize,
//         aggregate: Value,
//     ) -> SerializedSnapshot {
//         SerializedSnapshot {
//             aggregate_id,
//             aggregate,
//             current_sequence,
//             current_snapshot,
//         }
//     }
// }
