use std::sync::Arc;
use surrealdb::{Datastore, Session};

mod check_person_exists;
mod create_person;
mod delete_person;
mod find_persons;
mod find_person;
mod update_person;

pub use check_person_exists::*;
pub use create_person::*;
pub use delete_person::*;
pub use find_persons::*;
pub use find_person::*;
pub use update_person::*;

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
