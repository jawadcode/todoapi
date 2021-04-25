use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize)]
pub struct TodoInsert {
    pub user_id: uuid::Uuid,
    pub cat_id: uuid::Uuid,
    pub title: String,
    pub description: String,
}

#[derive(Serialize, FromRow)]
pub struct Todo {
    pub id: uuid::Uuid,
    pub user_id: uuid::Uuid,
    pub cat_id: uuid::Uuid,
    pub title: String,
    pub description: String,
    pub completed: bool,
    pub created_at: u64,
    pub updated_at: u64,
}
