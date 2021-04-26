use serde::Serialize;

pub mod internal_server;
pub mod validation;

#[derive(Serialize)]
#[serde(untagged)]
/// Represents all the different error categories
pub enum ErrorCategories {
    ValidationError(validation::ValidationError),
    InternalServerError(internal_server::InternalServerError),
}

/// The actual error
#[derive(Serialize)]
pub struct ApplicationError {
    pub kind: &'static str,
    pub body: ErrorCategories,
}

/// Wrapper struct so we can have `{"error":{...}}` instead of just `{...}`
#[derive(Serialize)]
pub struct Error {
    pub error: ApplicationError,
}
