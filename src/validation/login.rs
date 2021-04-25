use crate::errors::validation::{auth::ErrorVariants, ValidationError};
use crate::models::user::UserLogin;
use fancy_regex::Regex;
use std::str::FromStr;

impl super::Validate for UserLogin {
    fn validate(&self) -> Option<ValidationError> {
        let un_len = self.username.len();
        let em_len = self.email.len();
        let em_validator = Regex::new(r#"^[^@\s]+@[^@\s]+\.[^@\.\s]+$"#).unwrap();
        let pass_len = self.password.len();

        Some(crate::errors::validation::auth::variants_to_error(
            if un_len <= 2 || un_len > 128 {
                ErrorVariants::UsernameLength
            } else if em_len < 6 || em_len > 256 {
                ErrorVariants::EmailLength
            } else if !em_validator.is_match(&self.email).unwrap() {
                ErrorVariants::EmailInvalid
            } else if pass_len < 8 || pass_len > 256 {
                ErrorVariants::PasswordWeak
            } else {
                return None;
            },
        ))
    }
}
