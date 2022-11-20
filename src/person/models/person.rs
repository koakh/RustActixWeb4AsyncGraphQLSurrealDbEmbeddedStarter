use std::collections::BTreeMap;

use async_graphql::{
    connection::PageInfo, ComplexObject, Context, Enum, InputObject, Result, SimpleObject,
};
use log::debug;
use surrealdb::sql::Value;

use crate::{app::AppStateGlobal, db::add_filter_to_ast, relay::Base64Cursor};

#[derive(InputObject)]
pub struct InputFilter {
    pub id: Option<String>,
    pub name: Option<String>,
    pub age: Option<i8>,
}

#[derive(Enum, Copy, Clone, Eq, PartialEq)]
pub enum Order {
    Id,
    Name,
    Age,
}

// #[derive(InputObject)]
// pub struct InputOrder {
//     pub order: Order,
// }

#[derive(Debug, SimpleObject)]
pub struct Person {
    pub id: String,
    pub name: String,
    pub age: Option<i8>,
    pub meta_data: Option<MetaData>,
}

// from surrealdb value to struct
impl From<Value> for Person {
    fn from(value: Value) -> Self {
        // require to initialize model
        let mut model = Self {
            id: String::from(""),
            name: String::from(""),
            age: None,
            meta_data: None,
        };

        if let Value::Object(object) = value {
            // use into_inter this way we don't need to use the clone() inside loop, like if we use into()
            for (k, v) in object.into_iter() {
                // this will convert String to &str, nice improvement, a lot cleaner
                match &k[..] {
                    "id" => model.id = v.as_string(),
                    "name" => model.name = v.as_string(),
                    "age" => model.age = Some(v.as_int() as i8),
                    "meta_data" => model.meta_data = Some(MetaData::from(v)),
                    _ => {}
                }
            }
        }

        model
    }
}

#[derive(Debug, SimpleObject)]
pub struct MetaData {
    pub field: Option<String>,
}

// from surrealdb value to struct
impl From<Value> for MetaData {
    fn from(value: Value) -> Self {
        let mut model = Self { field: None };

        if let Value::Object(object) = value {
            for (k, v) in object.into_iter() {
                if let "field" = &k[..] {
                    model.field = Some(v.as_string());
                }
            }
        }
        model
    }
}

// relay stuff

#[derive(Debug, SimpleObject)]
#[graphql(complex)]
pub struct PersonConnection {
    // A list of edges.
    pub edges: Vec<PersonEdge>,
    // helper
    #[graphql(skip)]
    pub after: Option<String>,
    #[graphql(skip)]
    pub before: Option<String>,
    #[graphql(skip)]
    pub first: Option<i32>,
    #[graphql(skip)]
    pub last: Option<i32>,
}

#[ComplexObject]
impl PersonConnection {
    // Information to aid in pagination.
    async fn page_info(&self, ctx: &Context<'_>) -> Result<PageInfo> {
        // let server_ctx = ctx.data::<Arc<ServerContext>>()?;
        // let page_info = server_ctx
        //     .user_service
        //     .find_page_info(
        //         self.first,
        //         self.after.clone(),
        //         self.last,
        //         self.before.clone(),
        //     )
        //     .await?;
        // Ok(page_info.into())

        // TODO: add to notes destructure Struct with porp alias
        // destructure AppStateGlobal
        let AppStateGlobal {
            datastore: db,
            session: ses,
            counter: _,
            person_service: _,
        } = &ctx.data_unchecked::<AppStateGlobal>();

        // TODO: add to notes destructure Self
        // https://stackoverflow.com/questions/43603102/is-it-possible-to-destructure-the-self-argument-of-a-method
        let &PersonConnection {
            edges: _,
            after,
            before,
            first,
            last,
        } = &self;

        // TODO: here we get self that have first, after, last and before
        // we can use this to get has_next, has_previous_page,
        // and calculate cursors with base64 from example
        // debug!("first: {}", self.first.unwrap());
        // debug!(
        //     "first: {}, after: {}, last: {}, before: {}",
        //     self.first.unwrap(),
        //     self.after.clone().unwrap(),
        //     self.last.unwrap(),
        //     self.before.clone().unwrap()
        // );

        // TODO: add to add_filter_to_ast limit and start to use here

        // let mut ast = "SELECT * FROM person".to_string();
        // let mut vars = BTreeMap::new();
        // add_filter_to_ast(&filter, &ast, &vars);

        Ok(PageInfo {
            has_next_page: true,
            has_previous_page: true,
            // TODO: use base64 here?
            start_cursor: Some("start".to_string()),
            end_cursor: Some("end".to_string()),
        })

        // let mut has_next_query: String = String::new();
        // let mut has_next_page: bool = false;

        // match (first, after, last, before) {
        //     // First
        //     (Some(first), None, None, None) => {
        //         has_next_query = format!(
        //             r#"select count(*) > {first} from
        //              ( select "id" from user_ order by id asc limit {limit} )
        //            as data"#,
        //             limit = first + 1
        //         );
        //     }
        //     // First & after,
        //     (Some(first), Some(after), None, None) => {
        //         has_next_query = format!(
        //             r#"select count(*) > {first} from
        //              ( select "id" from user_ where id > '{after}' order by id asc limit {limit} )
        //            as data"#,
        //             limit = first + 1
        //         );
        //     }
        //     _ => (),
        // };

        // // has_next query
        // if let Some(_first) = first {
        //     has_next_page = match sqlx::query(&has_next_query).fetch_one(db).await {
        //         Err(err) => {
        //             error!("{}", &err);
        //             return Err(err.into());
        //         }
        //         Ok(row) => row.get(0),
        //     };
        // };

        // let (start_cursor, end_cursor) = if !rows.is_empty() {
        //     let start_cursor = Base64Cursor::new(rows[0].id).encode();
        //     let end_cursor = Base64Cursor::new(rows[rows.len() - 1].id).encode();
        //     (Some(start_cursor), Some(end_cursor))
        // } else {
        //     (None, None)
        // };

        // let has_previous_page = self.has_previous_page(rows, last).await?;
        // let page_info = PageInfo {
        //     has_next_page,
        //     has_previous_page,
        //     start_cursor,
        //     end_cursor,
        // };

        // Ok(page_info)
    }

    // pub async fn has_previous_page(
    //     &self,
    //     rows: &Vec<Person>,
    //     last: Option<i32>,
    // ) -> Result<bool, Error> {
    //     let mut has_previous_page: bool = false;
    //     if let Some(last) = last {
    //         debug!("rows length: {}. last: {}", rows.len(), last);
    //         has_previous_page = rows.len() > last as usize
    //     };
    //     Ok(has_previous_page)
    // }

    // Identifies the total count of items in the connection.
    async fn total_count(&self, ctx: &Context<'_>) -> Result<i64> {
        // let server_ctx = ctx.data::<Arc<ServerContext>>()?;
        // let db = &server_ctx.user_service.db;

        // let total_count_query = "select count(*) as exact_count from  user_";
        // match sqlx::query(total_count_query).fetch_one(db).await {
        //     Err(err) => {
        //         tracing::error!("{}", &err);
        //         Err(err.into())
        //     }
        //     Ok(row) => Ok(row.get(0)),
        // }

        let AppStateGlobal {
            datastore: db,
            session: ses,
            counter: _,
            person_service: _,
        } = &ctx.data_unchecked::<AppStateGlobal>();

        Ok(100)
    }
}

#[derive(Debug, SimpleObject)]
pub struct PersonEdge {
    // The item at the end of the edge.
    pub node: Person,
    // A cursor for use in pagination.
    pub cursor: String,
}

impl From<Person> for PersonEdge {
    fn from(person: Person) -> Self {
        let cursor = Base64Cursor::new(person.id.clone()).encode();
        let person_model = person.into();
        Self {
            node: person_model,
            cursor,
        }
    }
}