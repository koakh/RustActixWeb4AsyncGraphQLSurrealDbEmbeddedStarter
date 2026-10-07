use async_graphql::{EmptySubscription, MergedObject, Schema, EmptyMutation};

use crate::{person::resolver::PersonQuery, star_wars::StarWarsQuery};

#[derive(MergedObject, Default)]
pub struct Query(PersonQuery, StarWarsQuery /*MetaQuery, UserQuery, HealthQuery*/);

// #[derive(MergedObject, Default)]
// pub struct Mutation(UserMutation);

pub type AppSchema = Schema<Query, /*Mutation*/ EmptyMutation, EmptySubscription>;
