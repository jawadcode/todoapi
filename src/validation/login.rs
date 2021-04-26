use crate::errors::validation::{auth::ErrorVariants, ValidationError};
use crate::models::user::UserLogin;

impl super::Validate for UserLogin {
    /// Validates a login request body
    fn validate(&self) -> Option<ValidationError> {
        let un_len = self.username.len();
        let em_len = self.email.len();
        let pass_len = self.password.len();

        Some(ErrorVariants::to_validation_error(
            if un_len <= 2 || un_len > 128 {
                ErrorVariants::UsernameLength
            } else if em_len < 6 || em_len > 256 {
                ErrorVariants::EmailLength
            } else if !super::EMAIL_VALIDATOR.is_match(&self.email).unwrap() {
                ErrorVariants::EmailInvalid
            } else if pass_len < 8 || pass_len > 256 {
                ErrorVariants::PasswordWeak
            } else {
                return None;
            },
        ))
    }
}
