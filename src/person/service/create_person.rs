// use chrono::Utc;

use surrealdb::sql::Id;

use super::{Service};
use crate::{errors::app::Error, person::model::{Person, input::{CreatePersonInput}, MetaData}};

impl Service {
    pub async fn create_person(
        &self,
        input: CreatePersonInput,
    ) -> Result<Person, Error> {
        let username_exists = self.check_username_exists(&self.db, &self.ses, &input.name).await?;
        if username_exists {
            return Err(Error::UsernameAlreadyExists.into());
        }

        let person_input = Person {
            // TODO: must ommit this when create it in surrealdb
            id: String::from(""),
            name: input.name,
            age: input.age,
            // TODO:
            // inline convert MetaDataInput to MetaData
            meta_data: match input.meta_data {
                Some(v) => Some(MetaData { field: v.field }),
                None => None,
            },
            // created_at: Utc::now(),
            // updated_at: Utc::now(),
        };

        let person = self.repo.create_person(&self.db, &person_input).await?;

        Ok(person)
    }
}
