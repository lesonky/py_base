use crate::prelude::Result;
use crate::DBPool;
use async_trait::async_trait;

#[async_trait]
pub trait CrudEntity {
    type Id: Send;
    type CreateSchema: Send;
    type UpdateSchema: Send;
    type ListPageSchema: Send;

    async fn insert_one(&db: &DBPool, data: Self::CreateSchema) -> Result<Self::Id>;
    async fn update_one(&db: &DBPool, data: Self::UpdateSchema) -> Result<u64>;
    async fn delete_one(&db: &DBPool, data: Self::Id) -> Result<u64>;
    async fn find_by_id(&db: &DBPool, data: Self::Id) -> Result<Self>
    where
        Self: Sized;
    async fn find_page(&db: &DBPool, data: Self::ListPageSchema) -> Result<(Vec<Self>, u64)>
    where
        Self: Sized;
}
