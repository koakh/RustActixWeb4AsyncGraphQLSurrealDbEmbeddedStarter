use std::sync::Arc;
use surrealdb::{Datastore, Session};

mod find_persons;
pub use find_persons::*;

use super::repository::Repository;

pub struct Service {
    repo: Repository,
    pub db: Arc<Datastore>,
    pub ses: Arc<Session>,
}

impl Service {
    pub fn new(db: Arc<Datastore>, ses: Arc<Session>) -> Self {
        let repo = Repository::new();
        Self { db, ses, repo }
    }
}
