use serde::Serialize;
use std::{cell::Cell, sync::{Mutex, Arc}};
use surrealdb::{Datastore, Session};

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AppState {
    // Mutex is necessary to mutate safely across threads
    pub server_id: usize,
    // worker, used for every thread/workers
    pub request_count: Cell<usize>,
}

pub struct AppStateGlobal {
    // global, used for all workers
    pub counter: Mutex<i32>,
    // surrealdb
    pub datastore: Arc<Datastore>,
    pub session: Session,
}
