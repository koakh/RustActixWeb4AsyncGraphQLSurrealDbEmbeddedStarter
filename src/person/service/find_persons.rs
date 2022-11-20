use async_graphql::connection::PageInfo;

use crate::{errors::Error, person::model::PersonEdge};

use super::Service;
use crate::relay::validation::{convert_params, validate_params};

impl Service {
    pub async fn find_persons(
        &self,
        first: Option<i32>,
        after: Option<String>,
        last: Option<i32>,
        before: Option<String>,
    ) -> Result<Vec<PersonEdge>, Error> {
        validate_params(first, last)?;
        // let (after_uuid, before_uuid) = convert_params(after, before)?;
        let (after_id, before_id) = convert_params(after, before)?;

        let persons = self
            .repo
            .find_by_filter(&self.db, first, after_id, last, before_id)
            .await?;

        let person_edges: Vec<PersonEdge> =
            persons.into_iter().map(|person| person.into()).collect();

        Ok(person_edges)
    }

    pub async fn find_page_info(
        &self,
        first: Option<i32>,
        after: Option<String>,
        last: Option<i32>,
        before: Option<String>,
    ) -> Result<PageInfo, Error> {
        // let (after_uuid, before_uuid) = convert_params(after, before)?;
        let (after_id, before_id) = convert_params(after, before)?;

        let persons = self
            .repo
            // .find_by_filter(&self.db, first, after_uuid, last, before_uuid)
            .find_by_filter(&self.db, first, after_id.clone(), last, before_id.clone())
            .await?;

        let page_info = self
            .repo
            // .find_page_info(&self.db, &persons, first, after_uuid, last, before_uuid)
            .find_page_info(&self.db, &persons, first, after_id, last, before_id)
            .await?;
        Ok(page_info)
    }
}
