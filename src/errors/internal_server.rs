use serde::Serialize;
use std::{
    error::Error,
    fmt::{self, Display},
};

#[derive(Serialize, Debug)]
/// An internal server error (there is no message/body because the client doesn't need extra info)
pub struct InternalServerError {
    pub kind: &'static str,
}

const DB_ERROR: InternalServerError = InternalServerError { kind: "DBError" };

const AUTH_ERROR: InternalServerError = InternalServerError { kind: "AuthError" };

/// The variants of an internal server error
pub enum ErrorVariants {
    DBError,
    AuthError,
}

impl ErrorVariants {
    /// Wraps error variant in the `errors::Error` struct
    pub fn to_error(self) -> super::Error {
        super::Error {
            error: super::ApplicationError {
                kind: "InternalServerError",
                body: super::ErrorCategories::InternalServerError(match self {
                    ErrorVariants::DBError => DB_ERROR,
                    ErrorVariants::AuthError => AUTH_ERROR,
                }),
            },
        }
    }
}

impl Display for InternalServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "kind: {}", self.kind)
    }
}

impl Error for InternalServerError {}
