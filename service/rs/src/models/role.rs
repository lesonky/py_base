use serde::{Deserialize, Serialize};
use sqlx::types::JsonValue;
use sqlx::FromRow;

#[derive(Debug, FromRow, Clone, Deserialize, Serialize)]
pub struct Role {
    pub id: i64,
    pub name: String,
    pub permissions: Option<JsonValue>,
}

pub enum Permission {
    All = 0,
}

pub struct UpdateRoleSchema {
    pub id: u64,
    pub name: Option<String>,
    pub permissions: Option<JsonValue>,
}

pub struct InsertRoleSchema {
    pub name: String,
    pub permissions: Option<JsonValue>,
}
