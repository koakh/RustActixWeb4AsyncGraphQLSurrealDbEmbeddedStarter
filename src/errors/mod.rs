mod app;

pub use app::{Error::*};
use crate::relay;

use thiserror::Error;

#[derive(Error, Debug, Clone)]
pub enum Error {
    #[error("Internal error")]
    Internal(String),

    #[error("{0}")]
    NotFound(String),

    #[error("{0}")]
    PermissionDenied(String),

    #[error("{0}")]
    InvalidArgument(String),

    #[error("{0}")]
    AlreadyExists(String),
}

impl std::convert::From<relay::Base64CursorError> for Error {
  fn from(err: relay::Base64CursorError) -> Self {
      Error::InvalidArgument(err.to_string())
  }
}