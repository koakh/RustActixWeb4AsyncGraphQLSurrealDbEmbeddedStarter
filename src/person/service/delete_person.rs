use crate::{person::model::Person, errors::Error};

use super::Service;

impl Service {
    pub async fn delete_person(&self, person_id: String) -> Result<Person, Error> {
        let person = self
            .repo
            .delete_person(&self.db, &self.ses, person_id)
            .await?;

        Ok(person)
    }
}
