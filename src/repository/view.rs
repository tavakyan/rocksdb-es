use std::marker::PhantomData;

use async_trait::async_trait;
use cqrs_es::persist::{PersistenceError, ViewContext, ViewRepository};
use cqrs_es::{Aggregate, View};

/// A RockDB backed query repository for use in backing a `GenericQuery`.
pub struct RocksDbViewRepository<V, A> {
    _phantom: PhantomData<(V, A)>,
}
impl<V, A> RocksDbViewRepository<V, A>
where
    V: View<A>,
    A: Aggregate,
{
    pub fn new(view_name: &str, path: &str) -> Self {
        Self {
            _phantom: Default::default(),
        }
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
