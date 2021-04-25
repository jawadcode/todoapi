pub mod login;
pub mod registration;

type ValidationError = crate::errors::validation::ValidationError;

pub trait Validate {
    fn validate(&self) -> Option<crate::errors::validation::ValidationError>;
}
