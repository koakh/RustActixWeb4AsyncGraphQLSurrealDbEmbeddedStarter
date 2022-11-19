#[derive(Debug)]
pub enum Error {
    // Other
    Internal,
    MissingFirstAndLastPaginationArguments,
    PassedFirstAndLastPaginationArguments,

    // User
    UserNotFound,
    UsernameAlreadyExists,
}

impl std::convert::From<Error> for super::Error {
    fn from(err: Error) -> Self {
        match err {
            // Users
            Error::UserNotFound => super::Error::NotFound(String::from("user not found")),
            Error::UsernameAlreadyExists => {
                super::Error::AlreadyExists(String::from("username is already in use"))
            }

            // Other
            Error::Internal => super::Error::Internal(String::new()),
            Error::MissingFirstAndLastPaginationArguments => super::Error::InvalidArgument(
                "You must provide a `first` or `last` value to properly paginate the entity."
                    .to_string(),
            ),
            Error::PassedFirstAndLastPaginationArguments => super::Error::InvalidArgument(
                "Passing both `first` and `last` for pagination is not supported.".to_string(),
            ),
        }
    }
}