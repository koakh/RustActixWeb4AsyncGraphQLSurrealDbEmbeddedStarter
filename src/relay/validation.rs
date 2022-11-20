use uuid::Uuid;

use crate::{
    errors::app::Error::{
        MissingFirstAndLastPaginationArguments, PassedFirstAndLastPaginationArguments,
    },
    relay::base_64_cursor::Base64Cursor,
};

/// Parse `after` and `befor` to cursor
pub fn convert_params(
    after: Option<String>,
    before: Option<String>,
    // ) -> Result<(Option<Uuid>, Option<Uuid>), crate::errors::Error> {
) -> Result<(Option<String>, Option<String>), crate::errors::Error> {
    let (after_uuid, before_uuid) = match (after, before) {
        (None, None) => (None, None),
        (Some(after), Some(before)) => (
            Some(Base64Cursor::decode(&after)?.into()),
            Some(Base64Cursor::decode(&before)?.into()),
        ),
        (Some(after), None) => (Some(Base64Cursor::decode(&after)?.into()), None),
        (None, Some(before)) => (None, Some(Base64Cursor::decode(&before)?.into())),
    };
    Ok((after_uuid, before_uuid))
}

pub fn validate_params(first: Option<i32>, last: Option<i32>) -> Result<(), crate::errors::Error> {
    match (first, last) {
        (None, None) => return Err(MissingFirstAndLastPaginationArguments.into()),
        (Some(_), Some(_)) => return Err(PassedFirstAndLastPaginationArguments.into()),
        _ => (),
    };

    Ok(())
}
