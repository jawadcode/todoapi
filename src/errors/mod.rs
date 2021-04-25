use serde::Serialize;

pub mod internal_server;
pub mod validation;
#[derive(Serialize)]
pub enum ErrorCategory {
    ValidationError(validation::ValidationError),
    InternalServerError(internal_server::InternalServerError),
}

#[derive(Serialize)]
pub struct ApplicationError {
    pub kind: &'static str,
    pub body: ErrorCategory,
}

#[derive(Serialize)]
pub struct Error {
    pub error: ApplicationError,
}

impl Error {
    pub fn from_category(err: ErrorCategory) -> Self {
        Error {
            error: ApplicationError {
                kind: match &err {
                    ErrorCategory::ValidationError(_) => "ValidationError",
                    ErrorCategory::InternalServerError(_) => "InternalServerError",
                },
                body: err,
            },
        }
    }
}
