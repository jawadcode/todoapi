pub mod auth {
    use crate::errors::internal_server::{ErrorVariants, InternalServerError};
    use crate::models::user::*;
    use anyhow::Result;
    use rand::prelude::*;

    pub fn gen_refresh_token() -> String {
        let mut bytes: [u8; 48] = [0; 48];
        let mut rng = rand::thread_rng();
        rng.fill_bytes(&mut bytes);
        base64::encode_config(&bytes, base64::URL_SAFE)
    }

    pub fn gen_auth_token(user: UserSafe) -> Result<String, InternalServerError> {
        match UserClaims::from_user_safe(user).to_token() {
            Ok(t) => Ok(t),
            _ => Err(InternalServerError::from_variant(ErrorVariants::AuthError)),
        }
    }
}
