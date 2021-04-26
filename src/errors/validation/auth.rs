use serde::Serialize;

type ValidationError = super::ValidationError;

const DISPLAYNAME_LENGTH: ValidationError = ValidationError {
    field: "displayname",
    message: "displayname must be 3 to 128 characters in length",
};

const USERNAME_LENGTH: ValidationError = ValidationError {
    field: "username",
    message: "username must be 3 to 128 characters in length",
};

const EMAIL_LENGTH: ValidationError = ValidationError {
    field: "email",
    message: "email must be 6 to 256 characters in length",
};

const EMAIL_INVALID: ValidationError = ValidationError {
    field: "email",
    message: "email is invalid",
};

const PASSWORD_WEAK: ValidationError = ValidationError {
    field: "password",
    message: "password must contain at least: 1 upper case letter, 1 lower case letter, 1 number or special character and must be between 8 and 128 characters in length",
};

pub enum ErrorVariants {
    DisplaynameLength,
    UsernameLength,
    EmailLength,
    EmailInvalid,
    PasswordWeak,
}

impl ErrorVariants {
    pub fn to_validation_error(self) -> ValidationError {
        match self {
            ErrorVariants::DisplaynameLength => DISPLAYNAME_LENGTH,
            ErrorVariants::UsernameLength => USERNAME_LENGTH,
            ErrorVariants::EmailLength => EMAIL_LENGTH,
            ErrorVariants::EmailInvalid => EMAIL_INVALID,
            ErrorVariants::PasswordWeak => PASSWORD_WEAK,
        }
    }
}
