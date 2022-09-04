use crate::error::Result;
use crate::models::role::*;
use crate::DBPool;

impl Role {
    pub async fn find_by_id(db: &DBPool, id: i64) -> Result<Role> {
        let row = sqlx::query_as!(
            Role,
            r#"
            select
                id,
                permissons
            from role where id = ?
            "#,
            id
        )
        .fetch_one(db)
        .await?;
        Ok(row)
    }

    pub async fn find_all(db: &DBPool) -> Result<(Vec<Role>, i64)> {
        let total_count = sqlx::query_scalar!(r#"select count(*) from role"#)
            .fetch_one(db)
            .await?;
        let rows: Vec<Role> = sqlx::query_as!(
            Role,
            r#"
            select 
                id,
                permissons
            from role
            "#
        )
        .fetch_all(db)
        .await?;
        return Ok((rows, total_count as i64));
    }

    pub async fn delete_one(db: &DBPool, id: i64) -> Result<()> {
        unimplemented!()
    }
}
