use super::Service;
use crate::{errors::Error, person::model::Person};

impl Service {
    pub async fn find_person(&self, id: String) -> Result<Person, Error> {
        let person = self.repo.find_person_by_id(&self.db, &self.ses, id).await?;

        Ok(person)
    }
}
