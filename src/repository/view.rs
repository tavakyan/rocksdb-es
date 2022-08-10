use std::marker::PhantomData;

use async_trait::async_trait;
use cqrs_es::persist::{PersistenceError, ViewContext, ViewRepository};
use cqrs_es::{Aggregate, View};
use rocksdb::{DBWithThreadMode, MultiThreaded, SingleThreaded, DB};

/// A RockDB backed query repository for use in backing a `GenericQuery`.
pub struct RocksDbViewRepository<V, A> {
    db: DBWithThreadMode<SingleThreaded>,
    _phantom: PhantomData<(V, A)>,
}
impl<V, A> RocksDbViewRepository<V, A>
where
    V: View<A>,
    A: Aggregate,
{
    pub fn new(view_name: &str, path: &str) -> Self {
        /// TODO: Convert this to MultiThreaded
        let db = DB::open_default(path).unwrap();
        let _phantom = Default::default();
        Self { db, _phantom }
    }
}

#[async_trait]
impl<V, A> ViewRepository<V, A> for RocksDbViewRepository<V, A>
where
    V: View<A>,
    A: Aggregate,
{
    async fn load(&self, view_id: &str) -> Result<Option<V>, PersistenceError> {
        todo!()
    }

    async fn load_with_context(
        &self,
        view_id: &str,
    ) -> Result<Option<(V, ViewContext)>, PersistenceError> {
        todo!()
    }

    async fn update_view(&self, view: V, context: ViewContext) -> Result<(), PersistenceError> {
        todo!()
    }
}
