pub mod auth {
    use crate::errors::{auth, Error};
    use serde::Serialize;

    #[derive(Serialize)]
    /// Represents a success message
    pub struct SuccessMessage {
        pub message: &'static str,
    }

    // Check the password the user provided against the hash
    pub fn check_password_hash(password: String, hash: &str) -> Option<Error> {
        if !argon2::verify_encoded(hash, password.as_bytes()).unwrap() {
            Some(auth::ErrorVariants::IncorrectPassword.to_error())
        } else {
            None
        }
    }
}
