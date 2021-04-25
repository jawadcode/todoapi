use anyhow::Result;
use sqlx::postgres::PgPoolOptions;
use std::{env, fs};

// Intialise PostgreSQL and Redis connections
pub async fn init() -> Result<(sqlx::PgPool, r2d2::Pool<redis::Client>)> {
    // Create a DB Pool with a max of 4 connections
    let db_conn = env::var("DB_CONN").expect("DB_CONN env var unset");
    let db_pool = PgPoolOptions::new()
        .max_connections(6)
        .connect(&db_conn)
        .await?;
    // Read schema into string, split by double newlines and then execute each query
    let schema = fs::read_to_string("schema.sql")?;
    for query in schema.split("\n\n") {
        sqlx::query(query).execute(&db_pool).await?;
    }
    // Create redis connection
    let redis_conn = env::var("REDIS_CONN").expect("REDIS_CONN env var unset");
    let client = redis::Client::open(redis_conn)?;
    let redis_pool = r2d2::Pool::builder().build(client).unwrap();
    // Return pool for use by the API
    Ok((db_pool, redis_pool))
}
