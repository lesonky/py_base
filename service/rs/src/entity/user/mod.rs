use crate::error::Result;
use crate::models::user::*;
use crate::DBPool;
use sqlx::MySql;
use sqlx::QueryBuilder;
use uuid::Uuid;

impl User {
    pub async fn find_by_id(db: &DBPool, id: i64) -> Result<User> {
        let row = sqlx::query_as!(
            User,
            r#"
            select
                id,
                name,
                account_id
            from user where id = ?"#,
            id
        )
        .fetch_one(db)
        .await?;
        Ok(row)
    }

    pub async fn find_one(db: &DBPool, options: &QueryFilter<'_>) -> Result<User> {
        let row = sqlx::query_as!(
            User,
            r#"
            select 
              id,
              name,
              account_id
            from user
            where (? is null or id = ?)
              and (? is null or name = ?)
              and (? is null or account_id = ?)
              and (? is null or hashed_passwd = ?)
            limit 1"#,
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
        Ok(row)
    }

    pub async fn find_page(db: &DBPool, options: &QueryFilter<'_>) -> Result<(Vec<User>, i64)> {
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

        let rows: Vec<User> = sqlx::query_as!(
            User,
            r#"
            select
                id,
                name,
                account_id
            from user
            where (? is null or id = ?)
              and (? is null or name = ?)
              and (? is null or account_id = ?)
              and (? is null or hashed_passwd = ?)
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
            options.limit.unwrap_or(10),
            options.offset.unwrap_or(0)
        )
        .fetch_all(db)
        .await?;
        return Ok((rows, total_count));
    }

    pub async fn update_one(db: &DBPool, data: EditUserSchema) -> Result<u64> {
        let hashed_passwd = data.password;
        sqlx::query!(
            r#"
            update user
            set name = coalesce(?, user.name),
                hashed_passwd = coalesce(?, user.hashed_passwd)
            where id = ?
            "#,
            data.name,
            hashed_passwd,
            data.id
        )
        .execute(db)
        .await?;
        Ok(data.id)
    }

    pub async fn insert_one(db: &DBPool, data: CreateUserSchema) -> Result<u64> {
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
