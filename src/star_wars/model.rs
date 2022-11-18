use std::collections::BTreeMap;

use async_graphql::connection::{query, Connection, Edge, EmptyFields};
use async_graphql::{Context, Enum, FieldResult, Interface, Object};
use log::debug;
use surrealdb::sql::{thing, Id, Number, Strand, Thing};
use surrealdb::Session;
use surrealdb::{sql::Value, Datastore};
// use surrealdb::sql::thing;

use crate::app::AppStateGlobal;
use crate::db::{add_filter_to_ast, InputFilter, Order as PersonOrder, Person, PersonConnection, PersonEdge};

use super::StarWars;

/// One of the films in the Star Wars Trilogy
#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum Episode {
    /// Released in 1977.
    NewHope,
    /// Released in 1980.
    Empire,
    /// Released in 1983.
    Jedi,
}

pub struct Human(usize);

/// A humanoid creature in the Star Wars universe.
#[Object]
impl Human {
    /// The id of the human.
    async fn id(&self, ctx: &Context<'_>) -> &str {
        ctx.data_unchecked::<StarWars>().chars[self.0].id
    }

    /// The name of the human.
    async fn name(&self, ctx: &Context<'_>) -> &str {
        ctx.data_unchecked::<StarWars>().chars[self.0].name
    }

    /// The friends of the human, or an empty list if they have none.
    async fn friends(&self, ctx: &Context<'_>) -> Vec<Character> {
        ctx.data_unchecked::<StarWars>().chars[self.0]
            .friends
            .iter()
            .map(|id| Human(*id).into())
            .collect()
    }

    /// Which movies they appear in.
    async fn appears_in<'a>(&self, ctx: &'a Context<'_>) -> &'a [Episode] {
        &ctx.data_unchecked::<StarWars>().chars[self.0].appears_in
    }

    /// The home planet of the human, or null if unknown.
    async fn home_planet<'a>(&self, ctx: &'a Context<'_>) -> &'a Option<&'a str> {
        &ctx.data_unchecked::<StarWars>().chars[self.0].home_planet
    }
}

pub struct Droid(usize);

/// A mechanical creature in the Star Wars universe.
#[Object]
impl Droid {
    /// The id of the droid.
    async fn id(&self, ctx: &Context<'_>) -> &str {
        ctx.data_unchecked::<StarWars>().chars[self.0].id
    }

    /// The name of the droid.
    async fn name(&self, ctx: &Context<'_>) -> &str {
        ctx.data_unchecked::<StarWars>().chars[self.0].name
    }

    /// The friends of the droid, or an empty list if they have none.
    async fn friends(&self, ctx: &Context<'_>) -> Vec<Character> {
        ctx.data_unchecked::<StarWars>().chars[self.0]
            .friends
            .iter()
            .map(|id| Droid(*id).into())
            .collect()
    }

    /// Which movies they appear in.
    async fn appears_in<'a>(&self, ctx: &'a Context<'_>) -> &'a [Episode] {
        &ctx.data_unchecked::<StarWars>().chars[self.0].appears_in
    }

    /// The primary function of the droid.
    async fn primary_function<'a>(&self, ctx: &'a Context<'_>) -> &'a Option<&'a str> {
        &ctx.data_unchecked::<StarWars>().chars[self.0].primary_function
    }
}

pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn hero(
        &self,
        ctx: &Context<'_>,
        #[graphql(
            desc = "If omitted, returns the hero of the whole saga. If provided, returns the hero of that particular episode."
        )]
        episode: Episode,
    ) -> Character {
        // // let state = &ctx.data_unchecked::<Datastore>();
        // // TODO: add to rust notes destructure
        // let AppStateGlobal {
        //     datastore: db,
        //     session: ses,
        //     counter,
        // } = &ctx.data_unchecked::<AppStateGlobal>();
        // let ast = format!("SELECT * FROM {}", "person");
        // // CREATE person:tobie CONTENT { name: 'Tobie', meta_data: { field: 'value' } };
        // // CREATE person:jamie CONTENT { name: 'Jamie', meta_data: { field: 'value' } };
        // let res = db.execute(&ast, ses, None, false).await.unwrap();
        // let data = &res[0].result.as_ref().to_owned();
        // let value = data.unwrap().single().to_owned();
        // debug!("value: {:?}", value);
        // // @Tobie way
        // // that is, this conversion is whatever the implementation of From<T> for U chooses to do.
        // let person: Person = value.into();
        // debug!("value person: {:?}", person);

        if episode == Episode::Empire {
            Human(ctx.data_unchecked::<StarWars>().luke).into()
        } else {
            Droid(ctx.data_unchecked::<StarWars>().artoo).into()
        }
    }

    async fn human(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "id of the human")] id: String,
    ) -> Option<Human> {
        ctx.data_unchecked::<StarWars>().human(&id).map(Human)
    }

    async fn humans(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> FieldResult<Connection<usize, Human, EmptyFields, EmptyFields>> {
        let humans = ctx.data_unchecked::<StarWars>().humans().to_vec();

        query_characters(after, before, first, last, &humans)
            .await
            .map(|conn| conn.map_node(Human))
    }

    async fn droid(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "id of the droid")] id: String,
    ) -> Option<Droid> {
        ctx.data_unchecked::<StarWars>().droid(&id).map(Droid)
    }

    async fn droids(
        &self,
        ctx: &Context<'_>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
    ) -> FieldResult<Connection<usize, Droid, EmptyFields, EmptyFields>> {
        let droids = ctx.data_unchecked::<StarWars>().droids().to_vec();

        query_characters(after, before, first, last, &droids)
            .await
            .map(|conn| conn.map_node(Droid))
    }

    async fn person(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "id of the person")] id: String,
    ) -> Option<Person> {
        // let state = &ctx.data_unchecked::<Datastore>();
        // TODO: add to rust notes destructure with alias ex datastore to db
        let AppStateGlobal {
            datastore: db,
            session: ses,
            counter: _,
        } = &ctx.data_unchecked::<AppStateGlobal>();

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
            return None;
        }
        // TODO: @Tobie way use into() and From<T> (into is the value returned from From<T> implementation)
        // that is, this conversion is whatever the implementation of From<T> for U chooses to do.
        let person: Person = value.into();
        // debug!("value person: {:?}", person);

        // return graphql objectType
        Some(person)
    }

    #[allow(clippy::too_many_arguments)]
    async fn persons(
        &self,
        ctx: &Context<'_>,
        #[graphql(desc = "custom filter")] filter: Option<InputFilter>,
        // TODO: use order
        #[graphql(desc = "custom order")] order: Option<PersonOrder>,
        after: Option<String>,
        before: Option<String>,
        first: Option<i32>,
        last: Option<i32>,
        // ) -> Vec<Person> {
        // ) -> FieldResult<Connection<usize, Person, EmptyFields, EmptyFields>> {
    ) -> FieldResult<PersonConnection> {
        // ) -> FieldResult<Connection<usize, Person, EmptyFields, EmptyFields>> {
        let AppStateGlobal {
            datastore: db,
            session: ses,
            counter: _,
        } = &ctx.data_unchecked::<AppStateGlobal>();

        // query_persons(after, before, first, last, db, ses, filter)
        //     .await
        //     .map(|conn| conn.map_node(Person))

        // let server_ctx = ctx.data::<Arc<ServerContext>>()?;
        // let user_edges = server_ctx
        //     .user_service
        //     .find_users(first, after.clone(), last, before.clone())
        //     .await?;

        let persons = query_persons(first, &after, last, &before, db, ses, filter).await;
        // use into_iter to dereference
        // required a mplicit type to use collect
        let edges: Vec<PersonEdge> = persons
            .into_iter()
            .map(|person| PersonEdge {
                cursor: person.id.clone(),
                node: person,
            })
            .collect();

        let person_connection = PersonConnection {
            edges,
            after,
            before,
            first,
            last,
        };

        Ok(person_connection)
    }
}

#[derive(Interface)]
#[graphql(
    field(name = "id", type = "&str"),
    field(name = "name", type = "&str"),
    field(name = "friends", type = "Vec<Character>"),
    field(name = "appears_in", type = "&'ctx [Episode]")
)]

pub enum Character {
    Human(Human),
    Droid(Droid),
}

async fn query_characters(
    after: Option<String>,
    before: Option<String>,
    first: Option<i32>,
    last: Option<i32>,
    characters: &[usize],
) -> FieldResult<Connection<usize, usize, EmptyFields, EmptyFields>> {
    query(
        after,
        before,
        first,
        last,
        |after, before, first, last| async move {
            let mut start = 0usize;
            let mut end = characters.len();

            if let Some(after) = after {
                if after >= characters.len() {
                    return Ok(Connection::new(false, false));
                }
                start = after + 1;
            }

            if let Some(before) = before {
                if before == 0 {
                    return Ok(Connection::new(false, false));
                }
                end = before;
            }

            let mut slice = &characters[start..end];

            if let Some(first) = first {
                slice = &slice[..first.min(slice.len())];
                end -= first.min(slice.len());
            } else if let Some(last) = last {
                slice = &slice[slice.len() - last.min(slice.len())..];
                start = end - last.min(slice.len());
            }

            let mut connection = Connection::new(start > 0, end < characters.len());
            connection.append(
                slice
                    .iter()
                    .enumerate()
                    .map(|(idx, item)| Edge::new(start + idx, *item)),
            );

            FieldResult::Ok(connection)
        },
    )
    .await
}

// TODO: move to repository pattern
async fn query_persons(
    first: Option<i32>,
    after: &Option<String>,
    last: Option<i32>,
    before: &Option<String>,
    db: &Datastore,
    ses: &Session,
    filter: Option<InputFilter>,
    // ) -> FieldResult<Connection<usize, usize, EmptyFields, EmptyFields>> {
) -> Vec<Person> {
    let mut ast = "SELECT * FROM person".to_string();
    // init parameters btree
    let mut vars: BTreeMap<String, Value> = BTreeMap::new();
    // inject filter in ast
    add_filter_to_ast("person".to_string(), &filter, &mut ast, &mut vars);

    // execute query
    let res = db
        .execute(ast.as_str(), &ses, Some(vars), false)
        .await
        .unwrap();
    // get query execute result
    let data = &res[0].result.as_ref().to_owned();
    // get surrealdb value
    // using single() we will get only the first record,
    // to get array don't use single(), and after it is_array will be true
    // let value = data.unwrap().single().to_owned();
    let value = data.unwrap().to_owned();
    debug!("value is_object: {}", value.is_object());
    debug!("value is_array: {}", value.is_array());

    let mut vec: Vec<Person> = Vec::new();
    if let Value::Array(array) = value {
        // debug!("array: {:?}", array);
        array.into_iter().for_each(|value| {
            debug!("surreal value {:?}", value);
            let person: Person = value.into();
            vec.push(person);
        });
        // debug!("surreal vec {:?}", vec);
    }
    // resturn record vector
    vec
}
