use crate::errors::internal_server::{ErrorVariants, InternalServerError};
use anyhow::Result;
use argon2::{self, Config};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::{
    env,
    time::{self, SystemTime, UNIX_EPOCH},
};

pub type Pool = sqlx::PgPool;

#[derive(Deserialize)]
pub struct UserInsert {
    pub displayname: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct UserLogin {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, FromRow)]
pub struct User {
    pub id: uuid::Uuid,
    pub displayname: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: u64,
}

#[derive(Serialize, FromRow)]
pub struct UserSafe {
    pub id: uuid::Uuid,
    pub displayname: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub created_at: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct UserClaims {
    pub id: uuid::Uuid,
    pub displayname: String,
    pub username: String,
    pub email: String,
    pub exp: usize,
}

impl User {
    pub async fn insert(pool: &Pool, ins: UserInsert) -> Result<UserSafe, InternalServerError> {
        let secret: String = std::env::var("SECRET").expect("SECRET env var unset");
        let salt = secret.as_bytes();
        let config = Config::default();
        let hash = argon2::hash_encoded(ins.password.as_bytes(), salt, &config).unwrap();
        let created_at: i64 = match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) => n.as_secs() as i64,
            _ => panic!("Couldn't get current time"),
        };

        match sqlx::query_as!(
            UserSafe,
            "INSERT INTO users (displayname, username, email, password, created_at)
            VALUES ($1, $2, $3, $4, $5) RETURNING id, displayname, username, email, created_at",
            ins.displayname,
            ins.username,
            ins.email,
            hash,
            created_at,
        )
        .fetch_one(pool)
        .await
        {
            Ok(u) => Ok(u),
            _ => Err(InternalServerError::from_variant(ErrorVariants::DBError)),
        }
    }
}

use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};

const ACCESS_TOKEN_LIFE: usize = 1200;

impl UserClaims {
    pub fn from_user_safe(user: UserSafe) -> Self {
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap().as_secs();

        UserClaims {
            id: user.id,
            displayname: user.displayname.unwrap(),
            username: user.username.unwrap(),
            email: user.email.unwrap(),
            exp: (since_the_epoch as usize) + ACCESS_TOKEN_LIFE,
        }
    }
    pub fn to_token(self) -> Result<String> {
        let JWT_SECRET = env::var("JWT_SECRET").expect("JWT_SECRET env var unset");
        Ok(encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(JWT_SECRET.as_ref()),
        )?)
    }
}
