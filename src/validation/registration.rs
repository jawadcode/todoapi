use crate::errors::validation::{auth::ErrorVariants, ValidationError};
use crate::models::user::UserInsert;

impl super::Validate for UserInsert {
    /// Validates a user registration request body
    fn validate(&self) -> Option<ValidationError> {
        let dn_len = self.displayname.len();
        let un_len = self.username.len();
        let em_len = self.email.len();

        Some(ErrorVariants::to_validation_error(
            if dn_len <= 2 || dn_len > 128 {
                ErrorVariants::DisplaynameLength
            } else if un_len <= 2 || un_len > 128 {
                ErrorVariants::UsernameLength
            } else if em_len < 6 || em_len > 256 {
                ErrorVariants::EmailLength
            } else if !super::EMAIL_VALIDATOR.is_match(&self.email).unwrap() {
                ErrorVariants::EmailInvalid
            } else if !super::PASSWORD_VALIDATOR.is_match(&self.password).unwrap() {
                ErrorVariants::PasswordWeak
            } else {
                return None;
            },
        ))
    }
}
