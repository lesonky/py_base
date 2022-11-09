use super::role::Role;

use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Clone, Deserialize, Serialize, Default)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub account_id: String,
    pub role: Option<Role>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct CreateUserSchema {
    pub name: String,
    pub password: String,
}

#[derive(Default, Serialize, Deserialize)]
pub struct UpdateUserSchema {
    pub id: u64,
    pub name: Option<String>,
    pub password: Option<String>,
    pub role_id: Option<u64>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct UserListPageSchema {
    pub id: Option<u64>,
    pub name: Option<String>,
    pub account_id: Option<String>,
    pub hashed_passwd: Option<String>,
    pub role_id: Option<u64>,
    pub page_num: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Default)]
pub struct QueryFilter<'a> {
    pub id: Option<u64>,
    pub name: Option<&'a str>,
    pub account_id: Option<&'a str>,
    pub hashed_passwd: Option<&'a str>,
    pub role_id: Option<i64>,
    pub page_num: Option<u64>,
    pub page_size: Option<u64>,
}
