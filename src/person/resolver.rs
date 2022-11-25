use crate::{
    app::appstate::AppStateGlobal,
    person::model::{Person, PersonConnection, PersonEdge},
};
use async_graphql::{Context, Error, FieldResult, Object};

use super::model::input::{CreatePersonInput, UpdatePersonInput};

#[derive(Default)]
pub struct PersonQuery;

#[Object]
impl PersonQuery {
    pub async fn persons(
        &self,
        ctx: &Context<'_>,
        first: Option<i32>,
        after: Option<String>,
        last: Option<i32>,
        before: Option<String>,
    ) -> FieldResult<PersonConnection> {
        // // TODO: add to rust notes destructure
        // let AppStateGlobal {
        //     datastore: _,
        //     session: _,
        //     counter: _,
        //     person_service,
        // } = &ctx.data_unchecked::<AppStateGlobal>();
        let person_service = &ctx.data_unchecked::<AppStateGlobal>().person_service;

        let person_edges = person_service
            .find_persons(first, after.clone(), last, before.clone())
            .await?;

        let edges: Vec<PersonEdge> = person_edges
            .into_iter()
            // .map(|person| person.into())
            .collect();

        let person_connection = PersonConnection {
            edges,
            // relay
            after,
            before,
            first,
            last,
        };

        Ok(person_connection)
    }

    pub async fn person(&self, ctx: &Context<'_>, id: String) -> FieldResult<Person> {
        let person_service = &ctx.data_unchecked::<AppStateGlobal>().person_service;

        let result = person_service.find_person(id).await;
        match result {
            Ok(res) => Ok(res),
            Err(err) => Err(Error::new(err.to_string())),
        }
    }
}

// TODO:
#[derive(Default)]
pub struct PersonMutation;

#[Object]
impl PersonMutation {
    pub async fn create_person(
        &self,
        ctx: &Context<'_>,
        input: CreatePersonInput,
    ) -> FieldResult<Person> {
        let person_service = &ctx.data_unchecked::<AppStateGlobal>().person_service;

        let service_input = CreatePersonInput {
            name: input.name,
            age: input.age,
        };
        let result = person_service.create_person(service_input).await;
        match result {
            Ok(res) => Ok(res.into()),
            Err(err) => Err(Error::new(err.to_string())),
        }
    }

    pub async fn update_person(
        &self,
        ctx: &Context<'_>,
        input: UpdatePersonInput,
    ) -> FieldResult<Person> {
        let person_service = &ctx.data_unchecked::<AppStateGlobal>().person_service;

        let service_input = UpdatePersonInput {
            id: input.id,
            name: input.name,
            age: input.age,
        };
        let result = person_service.update_person(service_input).await;
        match result {
            Ok(res) => Ok(res.into()),
            Err(err) => Err(Error::new(err.to_string())),
        }
    }

    pub async fn delete_person(&self, ctx: &Context<'_>, id: String) -> FieldResult<Person> {
        let person_service = &ctx.data_unchecked::<AppStateGlobal>().person_service;

        let result = person_service.delete_person(id).await;
        match result {
            Ok(res) => Ok(res.into()),
            Err(err) => Err(Error::new(err.to_string())),
        }
    }
}
