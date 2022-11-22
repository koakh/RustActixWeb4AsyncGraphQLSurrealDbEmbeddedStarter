use async_graphql::connection::PageInfo;
use log::{debug, error};
use surrealdb::{sql::Value, Datastore, Session};
use uuid::Uuid;

use crate::{errors::Error, person::model::Person, relay::base_64_cursor::Base64Cursor};

use super::Repository;

impl Repository {
    pub async fn find_by_filter(
        &self,
        db: &Datastore,
        ses: &Session,
        first: Option<i32>,
        // after: Option<Uuid>,
        after: Option<String>,
        last: Option<i32>,
        // before: Option<Uuid>,
        before: Option<String>,
    ) -> Result<Vec<Person>, Error> {
        // TODO: add to constants
        let default_page_size = 10;
        let mut query: String = "SELECT * FROM person".to_string();

        match (first, after, last, before) {
            // First
            (Some(first), None, None, None) => {
                query = format!("{query} ORDER BY id ASC LIMIT {}", first);
            }
            // First & after,
            (Some(first), Some(after), None, None) => {
                query =
                    format!("{query} WHERE id > 'person:{after}' ORDER BY id ASC LIMIT {first}");
            }
            // Last
            (None, None, Some(last), None) => {
                query = format!(
                    // "select * from ( select * from user_ order by id desc limit {limit} ) as data order by id asc",
                    "SELECT * FROM person ORDER BY id DESC, name DESC LIMIT {limit}",
                    limit = last + 1
                );
            }
            // Last & before
            (None, None, Some(last), Some(before)) => {
                query = format!(
                // "select * from ( select * from user_ where id < '{before}' order by id desc limit {limit} ) as data order by id asc;", 
                "SELECT * FROM person WHERE id < '{before}' ORDER BY id DESC, name DESC LIMIT {limit}",
                limit = last + 1)
            }
            // Default page size
            _ => query = format!("{query} limit {}", default_page_size),
        };

        // let rows: Vec<Person> = Vec::new();

        // let mut rows = match sqlx::query_as::<_, Person>(&query)
        //     .fetch_all(db)
        //     .await
        // {
        //     Err(err) => {
        //         error!("{}", &err);
        //         return Err(err.into());
        //     }
        //     Ok(res) => res,
        // };

        // let res = db.execute(&ast, ses, Some(vars), false).await.unwrap();

        // let has_previous_page = self.has_previous_page(&rows, last).await?;
        // if last.is_some() {
        //     // The real value start from index 1. The 0 index only act as a sign for `has_previous_page`
        //     rows = if has_previous_page {
        //         rows[1..rows.len()].to_vec()
        //     } else {
        //         rows
        //     }
        // };

        // Ok(rows)

        // execute query
        let res = db.execute(query.as_str(), ses, None, false).await.unwrap();
        // get query execute result
        let data = &res[0].result.as_ref().to_owned();
        // get surrealdb value
        // using single() we will get only the first record,
        // to get array don't use single(), and after it is_array will be true
        // let value = data.unwrap().single().to_owned();
        let value = data.unwrap().to_owned();
        debug!("value is_object: {}", value.is_object());
        debug!("value is_array: {}", value.is_array());

        let mut rows: Vec<Person> = Vec::new();
        if let Value::Array(array) = value {
            debug!("array: {:?}", array);
            array.into_iter().for_each(|value| {
                debug!("surreal value {:?}", value);
                let person: Person = value.into();
                rows.push(person);
            });
            // debug!("surreal vec {:?}", vec);
        }
        // return record vector
        Ok(rows)
    }

    pub async fn has_previous_page(
        &self,
        rows: &Vec<Person>,
        last: Option<i32>,
    ) -> Result<bool, Error> {
        let mut has_previous_page: bool = false;
        if let Some(last) = last {
            debug!("rows length: {}. last: {}", rows.len(), last);
            has_previous_page = rows.len() > last as usize
        };
        Ok(has_previous_page)
    }

    #[allow(clippy::too_many_arguments)]
    pub async fn find_page_info(
        &self,
        db: &Datastore,
        ses: &Session,
        rows: &Vec<Person>,
        first: Option<i32>,
        // after: Option<Uuid>,
        after: Option<String>,
        last: Option<i32>,
        // before: Option<Uuid>,
        before: Option<String>,
    ) -> Result<PageInfo, Error> {
        let mut has_next_query: String = String::new();
        let mut has_next_page: bool = false;

        match (first, after, last, before) {
            // First
            (Some(first), None, None, None) => {
                // has_next_query = format!(
                //     r#"select count(*) > {first} from
                //    ( select "id" from user_ order by id asc limit {limit} )
                //  as data"#,
                //     limit = first + 1
                // );
                has_next_query = format!("SELECT count, count > {first} AS data FROM (SELECT id, count() as count FROM (SELECT id FROM person ORDER BY name ASC LIMIT {limit}) GROUP BY ALL);", limit = first + 1);
            }
            // First & after,
            (Some(first), Some(after), None, None) => {
                // has_next_query = format!(
                //     r#"select count(*) > {first} from
                //    ( select "id" from user_ where id > '{after}' order by id asc limit {limit} )
                //  as data"#,
                //     limit = first + 1
                // );
                has_next_query = format!("SELECT count, count > {first} AS data FROM (SELECT id, count() as count FROM (SELECT id FROM person WHERE name > '{after}' ORDER BY name ASC LIMIT {limit}) GROUP BY ALL);", limit = first + 1);
            }
            _ => (),
        };

        // has_next query
        // if let Some(_first) = first {
        //     has_next_page = match sqlx::query(&has_next_query).fetch_one(db).await {
        //         Err(err) => {
        //             error!("{}", &err);
        //             return Err(err.into());
        //         }
        //         Ok(row) => row.get(0),
        //     };
        // };

        if let Some(_) = first {
            // execute query
            let res = db.execute(&has_next_query, ses, None, false).await.unwrap();
            // get query execute result
            let data = &res[0].result.as_ref().to_owned();
            // get surrealdb value
            let value = data.unwrap().single().to_owned();

            if let Value::Object(object) = value {
                // use into_inter this way we don't need to use the clone() inside loop, like if we use into()
                for (k, v) in object.into_iter() {
                    // this will convert String to &str, nice improvement, a lot cleaner
                    match &k[..] {
                        "data" => {
                            // convert surrealdb value to i64
                            if let Ok(i) = v.to_string().parse::<bool>() {
                                has_next_page = i;
                            }
                        }
                        _ => {}
                    }
                }
            };
        };

        let (start_cursor, end_cursor) = if !rows.is_empty() {
            let start_cursor = Base64Cursor::new(rows[0].id.clone()).encode();
            let end_cursor = Base64Cursor::new(rows[rows.len() - 1].id.clone()).encode();
            (Some(start_cursor), Some(end_cursor))
        } else {
            (None, None)
        };

        let has_previous_page = self.has_previous_page(rows, last).await?;
        let page_info = PageInfo {
            has_next_page,
            has_previous_page,
            start_cursor,
            end_cursor,
        };

        Ok(page_info)
    }
}
