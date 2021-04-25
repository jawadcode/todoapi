use crate::errors::validation::{auth::ErrorVariants, ValidationError};
use crate::models::user::UserInsert;
use fancy_regex::Regex;
use std::str::FromStr;

impl super::Validate for UserInsert {
    fn validate(&self) -> Option<ValidationError> {
        let dn_len = self.displayname.len();
        let un_len = self.username.len();
        let em_len = self.email.len();
        let em_validator = Regex::new(r#"^[^@\s]+@[^@\s]+\.[^@\.\s]+$"#).unwrap();
        let pass_validator =
            Regex::new(r#"(?=^.{8,128}$)((?=.*\d)|(?=.*\W+))(?![.\n])(?=.*[A-Z])(?=.*[a-z]).*$"#)
                .unwrap();

        Some(crate::errors::validation::auth::variants_to_error(
            if dn_len <= 2 || dn_len > 128 {
                ErrorVariants::DisplaynameLength
            } else if un_len <= 2 || un_len > 128 {
                ErrorVariants::UsernameLength
            } else if em_len < 6 || em_len > 256 {
                ErrorVariants::EmailLength
            } else if !em_validator.is_match(&self.email).unwrap() {
                ErrorVariants::EmailInvalid
            } else if !pass_validator.is_match(&self.password).unwrap() {
                ErrorVariants::PasswordWeak
            } else {
                return None;
            },
        ))
    }
}
