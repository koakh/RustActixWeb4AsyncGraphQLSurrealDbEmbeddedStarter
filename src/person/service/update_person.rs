// use chrono::Utc;

use super::Service;
use crate::{
    errors::app::Error,
    person::model::{input::UpdatePersonInput, MetaData, Person},
};

impl Service {
    pub async fn update_person(&self, input: UpdatePersonInput) -> Result<Person, Error> {
        let username_exists = self
            .check_username_exists(&self.db, &self.ses, &input.name)
            .await?;
        if username_exists {
            return Err(Error::UsernameAlreadyExists.into());
        }

        // convert 
        // let meta_data = match input.meta_data {
        //     Some(v) => Some(MetaData { field: v.field }),
        //     None => None,
        // };

        let person_input = Person {
            id: input.id,
            name: input.name,
            age: input.age,
            // inline convert MetaDataInput to MetaData
            meta_data: match input.meta_data {
                Some(v) => Some(MetaData { field: v.field }),
                None => None,
            },
            // meta_data: Some(MetaData {
            //     field: input.meta_data.unwrap().field,
            // }),
            // TODO:
            // updated_at: Utc::now(),
            // created_at: Utc::now(),
        };

        let person = self.repo.update_person(&self.db, &self.ses, &person_input).await?;

        Ok(person)
    }
}
