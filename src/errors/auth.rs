use serde::Serialize;
use std::{error, fmt};

#[derive(Serialize, Debug)]
/// An authentication error
pub struct AuthError {
    pub kind: &'static str,
    pub message: &'static str,
}

const USERNAME_NOT_FOUND: AuthError = AuthError {
    kind: "UsernameNotFound",
    message: "User with specified username not found",
};

const INCORRECT_PASSWORD: AuthError = AuthError {
    kind: "IncorrectPassword",
    message: "Password is incorrect",
};

// The variants of an authentication error
pub enum ErrorVariants {
    UsernameNotFound,
    IncorrectPassword,
}

impl ErrorVariants {
    /// Wraps error variant in the `errors::Error` struct
    pub fn to_error(self) -> super::Error {
        super::Error {
            error: super::ApplicationError {
                kind: "AuthError",
                body: super::ErrorCategories::AuthError(match self {
                    ErrorVariants::UsernameNotFound => USERNAME_NOT_FOUND,
                    ErrorVariants::IncorrectPassword => INCORRECT_PASSWORD,
                }),
            },
        }
    }
}

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "AuthError {{ kind: {}, message: \"{}\" }}",
            self.kind, self.message
        )
    }
}

impl error::Error for AuthError {}
