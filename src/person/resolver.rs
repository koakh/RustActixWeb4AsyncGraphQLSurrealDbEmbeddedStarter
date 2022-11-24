use crate::{
    app::appstate::AppStateGlobal,
    person::model::{Person, PersonConnection, PersonEdge},
};
use async_graphql::{Context, Error, FieldResult, Object};

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
        // get AppStateGlobal
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
// #[derive(Default)]
// pub struct PersonMutation;

// #[Object]
// impl PersonMutation {
//     pub async fn create_person(
//         &self,
//         ctx: &Context<'_>,
//         input: model::input::CreatePersonInput,
//     ) -> FieldResult<model::Person> {
//         let server_ctx = ctx.data::<Arc<ServerContext>>()?;

//         let service_input = service::CreatePersonInput {
//             name: input.name,
//             full_name: input.full_name,
//         };
//         let result = server_ctx.person_service.create_person(service_input).await;
//         match result {
//             Ok(res) => Ok(res.into()),
//             Err(err) => Err(Error::new(err.to_string())),
//         }
//     }
//     pub async fn update_person(
//         &self,
//         ctx: &Context<'_>,
//         input: model::input::UpdatePersonInput,
//     ) -> FieldResult<model::Person> {
//         let server_ctx = ctx.data::<Arc<ServerContext>>()?;

//         let service_input = service::UpdatePersonInput {
//             id: input.id,
//             name: input.name,
//             full_name: input.full_name,
//         };
//         let result = server_ctx.person_service.update_person(service_input).await;
//         match result {
//             Ok(res) => Ok(res.into()),
//             Err(err) => Err(Error::new(err.to_string())),
//         }
//     }
//     pub async fn delete_person(&self, ctx: &Context<'_>, id: Id) -> FieldResult<model::Person> {
//         let server_ctx = ctx.data::<Arc<ServerContext>>()?;

//         let result = server_ctx.person_service.delete_person(id).await;
//         match result {
//             Ok(res) => Ok(res.into()),
//             Err(err) => Err(Error::new(err.to_string())),
//         }
//     }
// }
