use super::permission::Permission;
use serde::{Deserialize, Serialize, Serializer};
use sqlx::types::JsonValue;
use sqlx::FromRow;

#[derive(Debug, FromRow, Clone, Deserialize, Serialize)]
pub struct Role {
    pub id: i64,
    pub name: String,

    #[serde(serialize_with = "ser_permission_to_names")]
    pub permissions: Option<JsonValue>,
}

fn ser_permission_to_names<S>(x: &Option<JsonValue>, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let names = Permission::json_value_to_names(x);
    names.serialize(s)
}

pub struct UpdateRoleSchema {
    pub id: u64,
    pub name: Option<String>,
    pub permissions: Option<JsonValue>,
}

pub struct CreateRoleSchema {
    pub name: String,
    pub permissions: Option<JsonValue>,
}

pub struct ListRolePageSchema {}
