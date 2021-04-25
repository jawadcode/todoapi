use serde::Serialize;

pub mod auth;

#[derive(Serialize)]
pub struct ValidationError {
    pub field: &'static str,
    pub message: &'static str,
}
