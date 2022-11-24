use std::collections::BTreeMap;

use surrealdb::{Datastore, Session, sql::{Thing, Value, Id}};

use super::Repository;
use crate::{errors::app::Error, person::model::Person};

impl Repository {
    pub async fn find_person_by_id(
        &self,
        db: &Datastore,
        ses: &Session,
        id: String,
    ) -> Result<Person, Error> {
        // prepare query
        let ast = "SELECT * FROM $id".to_string();
        // type inference lets us omit an explicit type signature (which would be `BTreeMap<&str, &str>` in this example).
        let mut vars = BTreeMap::new();
        // when ork with id's we must use Thing struct
        vars.insert(
            "id".to_string(),
            Value::Thing(Thing {
                tb: "person".to_string(),
                id: { Id::String(id) },
            }),
        );
        // execute query
        let res = db.execute(&ast, ses, Some(vars), false).await.unwrap();
        // get query execute result
        let data = &res[0].result.as_ref().to_owned();
        // get surrealdb value
        let value = data.unwrap().single().to_owned();
        // debug!("value: {:?}", value);
        // check if we have any object, else is a empty record set and we must return None
        if !value.is_object() {
            return Err(Error::UserNotFound);
        }
        // TODO: @Tobie way use into() and From<T> (into is the value returned from From<T> implementation)
        // that is, this conversion is whatever the implementation of From<T> for U chooses to do.
        let person: Person = value.into();
        // debug!("value person: {:?}", person);

        // return graphql objectType
        Ok(person)
    }
}
