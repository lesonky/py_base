use super::error::ApiJsonResult;
use super::ApiContext;
use crate::api::json;
use crate::models::role::Role;
use axum::routing::{get, post};
use axum::{Extension, Json, Router};
use serde::Deserialize;

pub fn router() -> Router {
    Router::new()
        .route("/api/role/list", get(list_role))
        .route("/api/role/create", post(create_role))
        .route("/api/role/delete", post(delete_role))
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct ListPageResp {
    items: Vec<Role>,
    total: i64,
}

async fn list_role(Extension(ctx): Extension<ApiContext>) -> ApiJsonResult<ListPageResp> {
    let (items, total) = Role::find_all(&ctx.db).await?;
    let resp = ListPageResp { items, total };
    Ok(json(&resp))
}

#[derive(Deserialize, Debug, Clone)]
struct CreateRoleReq {
    name: String,
    permissons: Vec<String>,
}

async fn create_role(
    Extension(_ctx): Extension<ApiContext>,
    Json(_req): Json<CreateRoleReq>,
) -> ApiJsonResult<Role> {
    unimplemented!();
}

#[derive(Deserialize, Debug, Clone)]
struct DeleteRoleReq {
    id: i64,
}

async fn delete_role(
    Extension(_ctx): Extension<ApiContext>,
    Json(_req): Json<DeleteRoleReq>,
) -> ApiJsonResult<()> {
    unimplemented!();
}
