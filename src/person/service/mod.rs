mod find_persons;

use std::sync::Arc;

pub use find_persons::*;

use surrealdb::{Datastore, Session};

use super::repository::Repository;

pub struct Service {
    repo: Repository,
    pub db: Arc<Datastore>,
    pub ses: Arc<Session>,
}

impl Service {
    pub fn new(db: Arc<Datastore>, ses: Arc<Session>) -> Self {
        let repo = Repository::new();
        Self { db , ses, repo }
    }
}