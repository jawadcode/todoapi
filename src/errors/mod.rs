use serde::Serialize;
use std::io::Write;

pub mod auth;
pub mod internal_server;
pub mod validation;

#[derive(Serialize, Debug)]
#[serde(untagged)]
/// Represents all the different error categories
pub enum ErrorCategories {
    ValidationError(validation::ValidationError),
    InternalServerError(internal_server::InternalServerError),
    AuthError(auth::AuthError),
}

/// The actual error
#[derive(Serialize, Debug)]
pub struct ApplicationError {
    pub kind: &'static str,
    pub body: ErrorCategories,
}

/// Wrapper struct so we can have `{"error":{...}}` instead of just `{...}`
#[derive(Serialize, Debug)]
pub struct Error {
    pub error: ApplicationError,
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Error {{
            error: ApplicationError {{
                kind: {},
                body: {:#?}
            }},
        }}",
            self.error.kind, self.error.body
        )
    }
}

impl std::error::Error for Error {}
