use crate::errors::{auth, internal_server, Error};
use anyhow::Result;
use argon2::{self, Config};
use jsonwebtoken;
use lazy_static::lazy_static;
use rand::prelude::*;
use redis::{Commands, RedisError};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use std::{
    env,
    time::{SystemTime, UNIX_EPOCH},
};
use woothee::parser::Parser;

pub type Pool = sqlx::PgPool;

#[derive(Deserialize)]
/// User registration request body, can be directly sent to database
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
    pub created_at: i64,
}

lazy_static! {
    /// Secret for password hashing
    static ref SECRET: String = std::env::var("SECRET").expect("SECRET env var unset");
    /// JWT secret to generate the signature for a token
    static ref JWT_SECRET: String = env::var("JWT_SECRET").expect("JWT_SECRET env var unset");
    /// User Agent parser
    static ref UA_PARSER: Parser = Parser::new();
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
            internal_server::ErrorVariants::DBError.to_error()
        })
    }
    /// Get `User` by username or return `Error`
    pub async fn get_by_username(pool: &Pool, username: String) -> Result<User, Error> {
        sqlx::query_as!(
            User,
            "SELECT id, displayname, username, email, password, created_at FROM users
            WHERE username=$1",
            username,
        )
        .fetch_one(pool)
        .await
        .map_err(|e| {
            eprintln!("Database Error: {}", e);
            match e {
                sqlx::Error::RowNotFound => auth::ErrorVariants::UsernameNotFound.to_error(),
                _ => internal_server::ErrorVariants::AuthError.to_error(),
            }
        })
    }
}

#[derive(Serialize, FromRow)]
/// Full user row but with password (hash) omitted
pub struct UserSafe {
    pub id: uuid::Uuid,
    pub displayname: String,
    pub username: String,
    pub email: String,
    pub created_at: i64,
}

#[derive(Serialize, Deserialize, Clone)]
// Represents a user's session as stored in redis
pub struct UserSession {
    #[serde(skip_serializing, skip_deserializing)]
    pub id: uuid::Uuid,
    pub token: String,
    pub os: String,
    pub browser: String,
    pub expiry: u64,
}

const SESSION_LIFE: u64 = 24 * 60 * 60;

impl UserSession {
    /// Construct `UserSession` from user's id, a randomly generated session/refresh token and their
    /// useragent (as a means of rough identification) and return it
    pub fn new(id: uuid::Uuid, useragent: &str) -> Self {
        // Generate 48 byte long buffer of random bytes
        let mut bytes: [u8; 48] = [0; 48];
        let mut rng = rand::thread_rng();
        rng.fill_bytes(&mut bytes);
        // Convert the buffer to a base64 string
        let token = base64::encode_config(&bytes, base64::URL_SAFE);
        // Parse the user agent (into `Result` since useragent can be invalid)
        let parsed = UA_PARSER.parse(useragent);
        // Get the current epoch time and add on the session life
        let start = SystemTime::now();
        let expiry = start.duration_since(UNIX_EPOCH).unwrap().as_secs() + SESSION_LIFE;
        // If the useragent is invalid, populate the os and browser fields with unknown
        if let None = parsed {
            return UserSession {
                id,
                token,
                os: "Unknown".to_string(),
                browser: "Unknown".to_string(),
                expiry,
            };
        }
        // If we've reached this point then `parsed` should be `Ok` so return the (ideal) session
        // object
        let parsed = parsed.unwrap();
        UserSession {
            id,
            token,
            os: parsed.os.to_string(),
            browser: parsed.browser_type.to_string(),
            expiry,
        }
    }

    /// Save `self` to the appropriate redis session and return a session token
    pub fn set_session(&self, conn: &r2d2::Pool<redis::Client>) -> String {
        // Grab connection from connection pool
        let mut conn = conn.get().unwrap();
        // Grab the user's sessions
        let sessions: Result<String, RedisError> = conn.get(format!("sessions:{}", self.id));
        // If the value exists, then deserialise the JSON array from a string into
        // `Vec<UserSession>`, else, assign an empty vector
        let mut sessions: Vec<UserSession> = if let Ok(s) = sessions {
            serde_json::from_str(&s).unwrap()
        } else {
            vec![]
        };
        // Push the current session onto the vector of sessions
        sessions.push(self.clone());
        // Serialise the vector of sessions into a string, `SET` it on the appropriate key and
        // return a clone of the session/refresh token
        let sessions = serde_json::to_string(&sessions).unwrap();
        conn.set::<String, String, ()>(format!("sessions:{}", self.id), sessions)
            .unwrap();
        self.token.clone()
    }
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

/// Access JWT Life in seconds
const ACCESS_TOKEN_LIFE: usize = 20 * 60;

impl UserClaims {
    /// Converts a `User` object (returned from database) into a `UserClaims` object
    pub fn from_user(user: User) -> Self {
        // Get the unix epoch time
        let start = SystemTime::now();
        let since_the_epoch = start.duration_since(UNIX_EPOCH).unwrap().as_secs();

        UserClaims {
            id: user.id,
            displayname: user.displayname,
            username: user.username,
            email: user.email,
            exp: (since_the_epoch as usize) + ACCESS_TOKEN_LIFE,
        }
    }
    /// Converts/encodes `self` into a JWT string (to be used as the access token)
    pub fn to_token(self) -> String {
        jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &self,
            &jsonwebtoken::EncodingKey::from_secret(JWT_SECRET.as_ref()),
        )
        .unwrap()
    }
}
