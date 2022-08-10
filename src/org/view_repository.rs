// use std::marker::PhantomData;
// use cqrs_es::{Aggregate, View};
// use async_trait::async_trait;
// use cqrs_es::persist::{PersistenceError, ViewContext, ViewRepository};
//
// /// A postgres backed query repository for use in backing a `GenericQuery`.
// pub struct RocksDbViewRepository<V, A> {
//     // insert_sql: String,
//     // update_sql: String,
//     // select_sql: String,
//     // pool: Pool<Postgres>,
//     _phantom: PhantomData<(V, A)>,
// }
//
// impl<V, A> RocksDbViewRepository<V, A>
//     where
//         V: View<A>,
//         A: Aggregate,
// {
//
// }
//
// #[async_trait]
// impl<V, A> ViewRepository<V, A> for RocksDbViewRepository<V, A>
//     where
//         V: View<A>,
//         A: Aggregate,
// {
//     async fn load(&self, view_id: &str) -> Result<Option<V>, PersistenceError> {
//         todo!()
//     }
//
//     async fn load_with_context(&self, view_id: &str) -> Result<Option<(V, ViewContext)>, PersistenceError> {
//         todo!()
//     }
//
//     async fn update_view(&self, view: V, context: ViewContext) -> Result<(), PersistenceError> {
//         todo!()
//     }
// }
//
//
// // TODO: port tests if needed