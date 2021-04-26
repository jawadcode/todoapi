use crate::errors::validation::ValidationError;
use fancy_regex::Regex;
use lazy_static::lazy_static;

pub mod login;
pub mod registration;

lazy_static! {
    static ref EMAIL_VALIDATOR: Regex = Regex::new(r#"^[^@\s]+@[^@\s]+\.[^@\.\s]+$"#).unwrap();
    static ref PASSWORD_VALIDATOR: Regex =
        Regex::new(r#"(?=^.{8,128}$)((?=.*\d)|(?=.*\W+))(?![.\n])(?=.*[A-Z])(?=.*[a-z]).*$"#)
            .unwrap();
}

pub trait Validate {
    fn validate(&self) -> Option<ValidationError>;
}
