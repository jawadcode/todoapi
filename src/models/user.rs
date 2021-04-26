use crate::errors::{internal_server::ErrorVariants, Error};
use anyhow::Result;
use argon2::{self, Config};
use lazy_static::lazy_static;
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
/// User login request body
pub struct UserLogin {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, FromRow)]
/// The full representation for a user (should never be sent to client, use `UserSafe` instead)
pub struct User {
    pub id: uuid::Uuid,
    pub displayname: String,
    pub username: String,
    pub email: String,
    pub password: String,
    pub created_at: u64,
}

#[derive(Serialize, FromRow)]
/// Full user row but with password (hash) omitted
pub struct UserSafe {
    pub id: uuid::Uuid,
    pub displayname: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub created_at: Option<i64>,
}

#[derive(Serialize, Deserialize)]
/// Stores claims to generate a JWT with, for a user
pub struct UserClaims {
    pub id: uuid::Uuid,
    pub displayname: String,
    pub username: String,
    pub email: String,
    pub exp: usize,
}

lazy_static! {
    /// Secret for password hashing
    static ref SECRET: String = std::env::var("SECRET").expect("SECRET env var unset");
    /// JWT secret to generate the signature for a token
    static ref JWT_SECRET: String = env::var("JWT_SECRET").expect("JWT_SECRET env var unset");
}

impl User {
    /// Takes in a (validated) user registration request body and inserts it into the "users" table,
    /// returning the user row (minus the password) or a DBError
    pub async fn insert(pool: &Pool, ins: UserInsert) -> Result<UserSafe, Error> {
        // Hash password
        let salt = SECRET.as_bytes();
        let config = Config::default();
        let hash = argon2::hash_encoded(ins.password.as_bytes(), salt, &config).unwrap();
        // Generate timestamp
        let created_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;
        // Insert user into DB
        sqlx::query_as!(
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
        // The actual db error is only necessary to us and not the client so an Error of type DBError is sent back
        .map_err(|e| {
            eprintln!("Database Error: {}", e);
            ErrorVariants::DBError.to_error()
        })
    }
}

use jsonwebtoken::{encode, EncodingKey, Header};

/// Access JWT Life in seconds (20 minutes)
const ACCESS_TOKEN_LIFE: usize = 1200;

impl UserClaims {
    /// Converts a `UserSafe` object (returned from database) into a `UserClaims` object
    pub fn from_user_safe(user: UserSafe) -> Self {
        // Get the unix epoch time
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
    /// Converts/encodes the claims into a JWT string
    pub fn to_token(self) -> Result<String> {
        Ok(encode(
            &Header::default(),
            &self,
            &EncodingKey::from_secret(JWT_SECRET.as_ref()),
        )?)
    }
}
