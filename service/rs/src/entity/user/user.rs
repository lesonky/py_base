use crate::models::user::*;
use crate::prelude::*;
use crate::DBPool;
use async_trait::async_trait;
use futures::TryStreamExt;
use sqlx::MySql;
use sqlx::QueryBuilder;
use uuid::Uuid;

use serde::{Deserialize, Serialize};
use sqlx::types::JsonValue;

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

#[async_trait]
impl CrudEntity for User {
    type Id = u64;
    type CreateSchema = CreateUserSchema;
    type UpdateSchema = UpdateUserSchema;
    type ListPageSchema = UserListPageSchema;

    async fn find_by_id(db: &DBPool, id: Self::Id) -> Result<Self> {
        let row = sqlx::query_as!(
            UserFromQuery,
            r#"
            select
                user.id as id,
                user.name as name,
                user.account_id as account_id,
                role.id as role_id,
                role.name as role_name,
                role.permissions as role_permissions
            from user 
            left join role
                on user.role_id = role.id
            where user.id = ?"#,
            id
        )
        .fetch_one(db)
        .await?;
        Ok(row.into())
    }

    async fn insert_one(db: &DBPool, data: Self::CreateSchema) -> Result<u64> {
        let account_id = Uuid::new_v4();
        let row_id = sqlx::query_as!(
            User,
            r#"insert into user 
                (name, account_id, hashed_passwd)
                values (?, ?, ?)
            "#,
            data.name,
            account_id,
            data.password
        )
        .execute(db)
        .await?
        .last_insert_id();
        Ok(row_id)
    }

    async fn update_one(db: &DBPool, data: Self::UpdateSchema) -> Result<u64> {
        let hashed_passwd = data.password;
        let ret = sqlx::query!(
            r#"
            update user
            set name = coalesce(?, user.name),
                hashed_passwd = coalesce(?, user.hashed_passwd),
                role_id = coalesce(?, user.role_id)
            where id = ?
            "#,
            data.name,
            hashed_passwd,
            data.role_id,
            data.id,
        )
        .execute(db)
        .await?;
        Ok(ret.rows_affected())
    }

    async fn delete_one(db: &DBPool, id: Self::Id) -> Result<u64> {
        let ret = sqlx::query!(
            r#"
            delete 
                from user
            where user.id = ? 
                limit 1"#,
            id
        )
        .execute(db)
        .await?;
        Ok(ret.rows_affected())
    }

    async fn find_page(db: &DBPool, options: Self::ListPageSchema) -> Result<(Vec<Self>, u64)> {
        let total_count = sqlx::query_scalar!(
            r#"
            select 
                count(*)
            from user
            where (? is null or id = ?)
              and (? is null or name = ?)
              and (? is null or account_id = ?)
              and (? is null or hashed_passwd = ?)
            "#,
            options.id,
            options.id,
            options.name,
            options.name,
            options.account_id,
            options.account_id,
            options.hashed_passwd,
            options.hashed_passwd,
        )
        .fetch_one(db)
        .await?;

        let limit = options.page_size.unwrap_or(10);
        let offset = (options.page_num.unwrap_or(1) - 1) * limit;

        let rows: Vec<User> = sqlx::query_as!(
            UserFromQuery,
            r#"
            select 
                user.id as id,
                user.name as name,
                user.account_id as account_id,
                role.id as role_id,
                role.name as role_name,
                role.permissions as role_permissions
            from user
            left join role
                on user.role_id = role.id
            where (? is null or user.id = ?)
              and (? is null or user.name = ?)
              and (? is null or user.account_id = ?)
              and (? is null or user.hashed_passwd = ?)
            order by user.id desc
            limit ?
            offset ?
            "#,
            options.id,
            options.id,
            options.name,
            options.name,
            options.account_id,
            options.account_id,
            options.hashed_passwd,
            options.hashed_passwd,
            limit,
            offset
        )
        .fetch(db)
        .map_ok(|x| x.into())
        .try_collect()
        .await?;
        return Ok((rows, total_count as u64));
    }
}

impl User {
    pub async fn find_one(db: &DBPool, options: &QueryFilter<'_>) -> Result<User> {
        let row = sqlx::query_as!(
            UserFromQuery,
            r#"
            select
                user.id as id,
                user.name as name,
                user.account_id as account_id,
                role.id as role_id,
                role.name as role_name,
                role.permissions as role_permissions
            from user
            left join role
                on user.role_id = role.id
            where (? is null or user.id = ?)
              and (? is null or user.name = ?)
              and (? is null or user.account_id = ?)
              and (? is null or user.hashed_passwd = ?)
              and (? is null or user.role_id = ?)
            limit 1"#,
            options.id,
            options.id,
            options.name,
            options.name,
            options.account_id,
            options.account_id,
            options.hashed_passwd,
            options.hashed_passwd,
            options.role_id,
            options.role_id,
        )
        .fetch_one(db)
        .await?;
        Ok(row.into())
    }

    pub async fn bulk_insert(db: &DBPool, data: &[CreateUserSchema]) -> Result<()> {
        let mut qb: QueryBuilder<MySql> =
            QueryBuilder::new("insert into user (name, account_id, hashed_passwd) ");
        qb.push_values(data, |mut b, user| {
            let account_id = Uuid::new_v4();
            b.push_bind(user.name.clone())
                .push_bind(account_id)
                .push_bind(user.password.clone());
        });
        let query = qb.build();
        query.execute(db).await?;
        Ok(())
    }
}
