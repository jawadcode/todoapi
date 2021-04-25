use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize)]
pub struct CategoryInsert {
    pub user_id: uuid::Uuid,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, FromRow)]
pub struct Category {
    pub id: uuid::Uuid,
    pub user_ud: uuid::Uuid,
    pub name: String,
    pub description: String,
    pub created_at: u64,
    pub updated_at: u64,
}
