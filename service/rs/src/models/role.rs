use serde::{Deserialize, Serialize};
use sqlx::types::JsonValue;
use sqlx::FromRow;

#[derive(Debug, FromRow, Clone, Deserialize, Serialize)]
pub struct Role {
    pub id: i64,
    pub permissons: Option<JsonValue>,
}

pub enum Permission {
    All = 0,
}
