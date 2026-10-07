use surrealdb::{Datastore, Session};

use super::Service;
use crate::errors::app::Error;

impl Service {
    /// returns true if a username exists. false otherwise
    pub async fn check_username_exists(
        &self,
        db: &Datastore,
        ses: &Session,
        name: &str,
    ) -> Result<bool, Error> {
        let find_existing_username_res = self.repo.find_user_by_name(db, ses, name).await;
        match find_existing_username_res {
            Ok(_) => Ok(true),
            Err(Error::UserNotFound) => Ok(false),
            Err(err) => Err(err.into()),
        }
    }
}
