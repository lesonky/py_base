use crate::models::user::role::*;
use crate::prelude::*;
use crate::DBPool;

impl Role {
    pub async fn find_by_id(db: &DBPool, id: u64) -> Result<Role> {
        let row = sqlx::query_as!(
            Role,
            r#"
            select
                id,
                name,
                permissions
            from role where id = ?
            "#,
            id
        )
        .fetch_one(db)
        .await?;
        Ok(row.into())
    }

    pub async fn find_all(db: &DBPool) -> Result<(Vec<Role>, u64)> {
        let total_count = sqlx::query_scalar!(r#"select count(*) from role"#)
            .fetch_one(db)
            .await?;
        let rows: Vec<Role> = sqlx::query_as!(
            Role,
            r#"
            select 
                id,
                name,
                permissions
            from role
            "#
        )
        .fetch_all(db)
        .await?;
        return Ok((rows, total_count as u64));
    }

    pub async fn delete_one(db: &DBPool, id: u64) -> Result<u64> {
        let ret = sqlx::query!(
            r#"
            delete 
                from role 
            where role.id = ? 
                limit 1"#,
            id
        )
        .execute(db)
        .await?;
        Ok(ret.rows_affected())
    }

    pub async fn update_one(db: &DBPool, data: UpdateRoleSchema) -> Result<u64> {
        let ret = sqlx::query!(
            r#"
            update role
            set 
                name = coalesce(?, role.name),
                permissions = coalesce(?, role.permissions)
            where id = ?
            "#,
            data.name,
            data.permissions,
            data.id
        )
        .execute(db)
        .await?;
        Ok(ret.rows_affected())
    }

    pub async fn insert_one(db: &DBPool, data: InsertRoleSchema) -> Result<u64> {
        let row_id = sqlx::query_as!(
            Role,
            r#"
            insert into role 
            (name, permissions)
            values (?, ?)
            "#,
            data.name,
            data.permissions
        )
        .execute(db)
        .await?
        .last_insert_id();
        Ok(row_id)
    }
}
