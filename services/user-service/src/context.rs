use common::client::cache_redis::Cache;
use mongodb::Database;
use std::sync::Arc;

/// The AppContext contains all the global data commonly used in the vast
/// majority of request handlers.
#[derive(Debug)]
pub struct AppContext {
    pub(crate) db: Arc<Database>,
    pub(crate) cache: Arc<Cache>,
}

impl AppContext {
    /// A MongoDB reference to the underlying database. Used to interract with
    /// collections, etc.
    pub fn db(&self) -> &Database {
        &self.db
    }

    /// A Redis cache pool.
    pub fn cache(&self) -> &Cache {
        &self.cache
    }
}
