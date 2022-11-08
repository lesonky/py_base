use crate::models::user::role::{InsertRoleSchema, Role, UpdateRoleSchema};
use crate::models::user::Permission;
use crate::prelude::*;
use axum::routing::{get, post};
use axum::{Extension, Json, Router};
use serde::{Deserialize, Serialize};

pub fn router() -> Router {
    Router::new()
        .route("/api/role/list", get(list_role))
        .route("/api/role/create", post(create_role))
        .route("/api/role/delete", post(delete_role))
        .route("/api/role/update", post(update_role))
        .route("/api/permission/list", get(list_permisson))
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct ListPageResp {
    items: Vec<Role>,
    total: i64,
}

async fn list_role(Extension(ctx): Extension<ApiContext>) -> ApiJsonResult<ListPageResp> {
    let (items, total) = Role::find_all(&ctx.db).await?;
    let resp = ListPageResp { items, total };
    resp.into_ok_json()
}

#[derive(Deserialize, Debug, Clone)]
struct CreateRoleReq {
    name: String,
    permissions: Vec<String>,
}

async fn create_role(
    Extension(ctx): Extension<ApiContext>,
    Json(req): Json<CreateRoleReq>,
) -> ApiJsonResult<Role> {
    let permissions = Permission::to_json_value(&req.permissions);
    let data = InsertRoleSchema {
        name: req.name,
        permissions: Some(permissions),
    };
    let role_id = Role::insert_one(&ctx.db, data).await?;
    let role = Role::find_by_id(&ctx.db, role_id).await?;
    role.into_ok_json()
}

#[derive(Deserialize, Debug, Clone)]
struct DeleteRoleReq {
    id: i64,
}

async fn delete_role(
    Extension(ctx): Extension<ApiContext>,
    Json(req): Json<DeleteRoleReq>,
) -> ApiJsonResult<u64> {
    let rows_affected = Role::delete_one(&ctx.db, req.id).await?;
    rows_affected.into_ok_json()
}

#[derive(Deserialize, Debug, Clone)]
struct UpdateRoleReq {
    id: u64,
    name: Option<String>,
    permissions: Option<Vec<String>>,
}

async fn update_role(
    Extension(ctx): Extension<ApiContext>,
    Json(req): Json<UpdateRoleReq>,
) -> ApiJsonResult<u64> {
    let permissions = req.permissions.map(|x| Permission::to_json_value(&x));
    let data = UpdateRoleSchema {
        id: req.id,
        name: req.name,
        permissions,
    };
    let rows_affected = Role::update_one(&ctx.db, data).await?;
    rows_affected.into_ok_json()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ListPermissionResp {
    permissions: Vec<String>,
}

async fn list_permisson() -> ApiJsonResult<ListPermissionResp> {
    let permissions = Permission::all_labels();
    let resp = ListPermissionResp { permissions };
    resp.into_ok_json()
}
