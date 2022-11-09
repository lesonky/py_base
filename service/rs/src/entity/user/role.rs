use crate::models::user::role::*;
use crate::prelude::*;
use crate::DBPool;
use async_trait::async_trait;

#[async_trait]
impl CrudEntity for Role {
    type Id = u64;
    type CreateSchema = CreateRoleSchema;
    type UpdateSchema = UpdateRoleSchema;
    type ListPageSchema = ListRolePageSchema;

    async fn insert_one(db: &DBPool, data: Self::CreateSchema) -> Result<u64> {
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

    async fn find_by_id(db: &DBPool, id: Self::Id) -> Result<Role> {
        let row = sqlx::query_as!(
            Role,
            r#"
            select
                id,
                name,
                permissions
            from role 
            where 
                id = ?
            "#,
            id
        )
        .fetch_one(db)
        .await?;
        Ok(row.into())
    }

    async fn update_one(db: &DBPool, data: UpdateRoleSchema) -> Result<u64> {
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

    async fn find_page(db: &DBPool, _data: Self::ListPageSchema) -> Result<(Vec<Role>, u64)> {
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

    async fn delete_one(db: &DBPool, id: Self::Id) -> Result<u64> {
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
}

impl Role {
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
}
