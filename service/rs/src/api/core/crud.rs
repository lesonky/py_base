use crate::prelude::*;
use crate::schemas::*;
use crate::DBPool;
use async_trait::async_trait;
use axum::routing::{get, post};
use axum::Router;
use axum::{extract::Query, Extension, Json};
use serde::{Deserialize, Serialize};

use super::ApiJsonResult;

#[async_trait]
pub trait CrudAPI {
    type CreateReq: Into<<<Self as CrudAPI>::EntityModel as CrudEntity>::CreateSchema> + Send;
    type ListPageReq: Into<<<Self as CrudAPI>::EntityModel as CrudEntity>::ListPageSchema> + Send;
    type UpdateReq: Into<<<Self as CrudAPI>::EntityModel as CrudEntity>::UpdateSchema> + Send;
    type DetailReq: Into<<<Self as CrudAPI>::EntityModel as CrudEntity>::Id> + Send;
    type DeleteReq: Into<<<Self as CrudAPI>::EntityModel as CrudEntity>::Id> + Send;
    type EntityModel: CrudEntity + Serialize;

    //list page
    async fn list_page(
        Extension(ctx): Extension<ApiContext>,
        Query(req): Query<Self::ListPageReq>,
    ) -> ApiJsonResult<ListPageResp<Self::EntityModel>> {
        let (items, total) = Self::EntityModel::find_page(&ctx.db, req.into()).await?;
        let resp = ListPageResp { items, total };
        resp.into_ok_json()
    }
    async fn create(
        Extension(ctx): Extension<ApiContext>,
        Json(req): Json<Self::CreateReq>,
    ) -> ApiJsonResult<Self::EntityModel> {
        let id = Self::EntityModel::insert_one(&ctx.db, req.into()).await?;
        let item = Self::EntityModel::find_by_id(&ctx.db, id).await?;
        item.into_ok_json()
    }

    async fn detail(
        Extension(ctx): Extension<ApiContext>,
        Query(req): Query<Self::DetailReq>,
    ) -> ApiJsonResult<Self::EntityModel> {
        let item = Self::EntityModel::find_by_id(&ctx.db, req.into()).await?;
        item.into_ok_json()
    }

    async fn update(
        Extension(ctx): Extension<ApiContext>,
        Json(req): Json<Self::UpdateReq>,
    ) -> ApiJsonResult<u64> {
        let item = Self::EntityModel::update_one(&ctx.db, req.into()).await?;
        item.into_ok_json()
    }

    async fn delete(
        Extension(ctx): Extension<ApiContext>,
        Json(req): Json<Self::DeleteReq>,
    ) -> ApiJsonResult<u64> {
        let ret = Self::EntityModel::delete_one(&ctx.db, req.into()).await?;
        ret.into_ok_json()
    }
}
