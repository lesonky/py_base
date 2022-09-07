use super::role::Role;
use super::Permission;
use serde::{Deserialize, Serialize};
use sqlx::types::JsonValue;
use sqlx::FromRow;

#[derive(Debug, FromRow, Clone, Deserialize, Serialize, Default)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub account_id: String,
    pub role: Option<Role>,
}

pub struct CreateUserSchema {
    pub name: String,
    pub password: String,
}

#[derive(Default, Serialize, Deserialize)]
pub struct EditUserSchema {
    pub id: i64,
    pub name: Option<String>,
    pub password: Option<String>,
    pub role_id: Option<u64>,
}

#[derive(Default)]
pub struct QueryFilter<'a> {
    pub id: Option<i64>,
    pub name: Option<&'a str>,
    pub account_id: Option<&'a str>,
    pub hashed_passwd: Option<&'a str>,
    pub role_id: Option<i64>,
    pub page_num: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Serialize, Deserialize)]
pub struct UserFromQuery {
    pub id: i64,
    pub name: String,
    pub account_id: String,
    pub role_id: Option<i64>,
    pub role_name: Option<String>,
    pub role_permissions: Option<JsonValue>,
}

impl From<UserFromQuery> for User {
    fn from(row: UserFromQuery) -> Self {
        let role = if let Some(role_id) = row.role_id {
            Some(Role {
                id: role_id,
                name: row.role_name.unwrap(),
                permissions: row.role_permissions,
            })
        } else {
            None
        };
        Self {
            id: row.id,
            name: row.name,
            account_id: row.account_id,
            role,
        }
    }
}
