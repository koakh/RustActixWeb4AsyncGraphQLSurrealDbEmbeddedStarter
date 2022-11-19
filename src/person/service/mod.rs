mod find_persons;

use std::sync::Arc;

pub use find_persons::*;

use surrealdb::Datastore;

use super::Repository;

pub struct Service {
    repo: Repository,
    pub db: Arc<Datastore>,
}

impl Service {
    pub fn new(db: Arc<Datastore>) -> Self {
        let repo = Repository::new();
        Self { db , repo }
    }
}