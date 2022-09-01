use crate::error::{Result, Error};

use sqlx::FromRow;
use serde::{Serialize, Deserialize};
use sqlx::mysql::MySqlPool as DBPool;

#[derive(Debug, FromRow, Clone, Deserialize, Serialize)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub account_id: String
}

#[derive(Default)]
pub struct FilterOptions<'a> {
    pub id: Option<i64>,
    pub name: Option<&'a str>,
    pub account_id: Option<&'a str>,
    pub hashed_passwd: Option<&'a str>,
}

impl User {
    pub async fn find_one(db: &DBPool, options: &FilterOptions<'_>) -> Result<User> {
        let row = sqlx::query_file_as!(
            User,
            "sqls/user/find_one.sql",
            options.id, options.id,
            options.name, options.name,
            options.account_id, options.account_id,
            options.hashed_passwd, options.hashed_passwd,
        ).fetch_one(db).await.map_err(|_|Error::RowNotFound);
        row
    }
}
