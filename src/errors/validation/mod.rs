use serde::Serialize;

pub mod auth;

#[derive(Serialize)]
/// A validation error, with `field` being the field of the struct that validation failed on and `message` containing the requirements that were not satisfied
pub struct ValidationError {
    pub field: &'static str,
    pub message: &'static str,
}

impl ValidationError {
    /// Wraps error variant in the `errors::Error` struct
    pub fn to_error(self) -> super::Error {
        super::Error {
            error: super::ApplicationError {
                kind: "ValidationError",
                body: super::ErrorCategories::ValidationError(self),
            },
        }
    }
}
