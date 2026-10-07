mod find_by_filter;
mod find_person_by_id;

pub use find_by_filter::*;
pub use find_person_by_id::*;

#[derive(Debug, Clone)]
pub struct Repository {}

impl Repository {
    pub fn new() -> Repository {
        Repository {}
    }
}

impl Default for Repository {
    fn default() -> Self {
        Self::new()
    }
}