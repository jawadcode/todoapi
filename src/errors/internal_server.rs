use serde::Serialize;
use std::{
    error::Error,
    fmt::{self, Display},
};

#[derive(Serialize, Debug)]
pub struct InternalServerError {
    pub kind: &'static str,
}

const DB_ERROR: InternalServerError = InternalServerError { kind: "DBError" };

const AUTH_ERROR: InternalServerError = InternalServerError { kind: "AuthError" };

pub enum ErrorVariants {
    DBError,
    AuthError,
}

impl InternalServerError {
    pub fn from_variant(err: ErrorVariants) -> Self {
        match err {
            ErrorVariants::DBError => DB_ERROR,
            ErrorVariants::AuthError => AUTH_ERROR,
        }
    }
}

impl Display for InternalServerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "kind: {}", self.kind)
    }
}

impl Error for InternalServerError {}
