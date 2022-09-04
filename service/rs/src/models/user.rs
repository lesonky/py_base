use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub account_id: String,
}

pub struct CreateUserSchema {
    pub name: String,
    pub password: String,
}

#[derive(Default, Serialize, Deserialize)]
pub struct EditUserSchema {
    pub id: u64,
    pub name: Option<String>,
    pub password: Option<String>,
}

#[derive(Default)]
pub struct QueryFilter<'a> {
    pub id: Option<i64>,
    pub name: Option<&'a str>,
    pub account_id: Option<&'a str>,
    pub hashed_passwd: Option<&'a str>,
    pub page_num: Option<u64>,
    pub page_size: Option<u64>,
}
