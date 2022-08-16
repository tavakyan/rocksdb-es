use std::marker::PhantomData;
use std::path::PathBuf;
use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use cqrs_es::persist::{PersistenceError, ViewContext, ViewRepository};
use cqrs_es::{Aggregate, View};
use rocksdb::{DBWithThreadMode, MultiThreaded, Options, SingleThreaded, DB};

/// A RockDB backed query repository for use in backing a `GenericQuery`.
pub struct RocksDbViewRepository<V, A> {
    db: Arc<RwLock<DB>>,
    _phantom: PhantomData<(V, A)>,
}
impl<V, A> RocksDbViewRepository<V, A>
where
    V: View<A>,
    A: Aggregate,
{
    pub fn new(view_name: &str, path: PathBuf) -> Self {
        /// TODO: Convert this to MultiThreaded
        let db = DB::open_default(path).unwrap();
        let db = Arc::new(RwLock::new(db));
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
