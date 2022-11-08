use super::schemas::*;
use crate::models::user::role::{InsertRoleSchema, Role, UpdateRoleSchema};
use crate::models::user::Permission;
use crate::prelude::*;
use crate::schemas::*;
use axum::routing::{get, post};
use axum::{Extension, Json, Router};

pub fn router() -> Router {
    Router::new()
        .route("/api/role/list", get(RoleAPI::list))
        .route("/api/role/create", post(RoleAPI::create))
        .route("/api/role/delete", post(RoleAPI::delete))
        .route("/api/role/update", post(RoleAPI::update))
        .route("/api/permission/list", get(RoleAPI::list_permisson))
}

pub struct RoleAPI {}

impl RoleAPI {
    async fn list(Extension(ctx): Extension<ApiContext>) -> ApiJsonResult<ListPageResp<Role>> {
        let (items, total) = Role::find_all(&ctx.db).await?;
        let resp = ListPageResp { items, total };
        resp.into_ok_json()
    }

    async fn create(
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

    async fn delete(
        Extension(ctx): Extension<ApiContext>,
        Json(req): Json<IDReq>,
    ) -> ApiJsonResult<u64> {
        let rows_affected = Role::delete_one(&ctx.db, req.id).await?;
        rows_affected.into_ok_json()
    }

    async fn update(
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

    async fn list_permisson() -> ApiJsonResult<ListPermissionResp> {
        let permissions = Permission::all_labels();
        let resp = ListPermissionResp { permissions };
        resp.into_ok_json()
    }
}
