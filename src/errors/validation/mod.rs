use serde::Serialize;
use std::{error, fmt};

pub mod auth;

#[derive(Serialize, Debug)]
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

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "ValidationError {{ field: {}, message: \"{}\" }}",
            self.field, self.message
        )
    }
}

impl error::Error for ValidationError {}
