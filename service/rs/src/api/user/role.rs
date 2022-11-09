use super::schemas::*;
use crate::prelude::*;
use crate::schemas::*;
use axum::routing::{get, post};
use axum::Router;

use crate::{
    models::user::{CreateRoleSchema, ListRolePageSchema, Permission, Role, UpdateRoleSchema},
    schemas::ListPageReq,
};

pub fn router() -> Router {
    Router::new()
        .route("/api/role/list", get(RoleAPI::list_page))
        .route("/api/role/create", post(RoleAPI::create))
        .route("/api/role/delete", post(RoleAPI::delete))
        .route("/api/role/update", post(RoleAPI::update))
        .route("/api/permission/list", get(RoleAPI::list_permisson))
}

pub struct RoleAPI {}

impl CrudAPI for RoleAPI {
    type EntityModel = Role;
    type CreateReq = CreateRoleReq;
    type UpdateReq = UpdateRoleReq;
    type ListPageReq = ListPageReq;
    type DetailReq = IDReq;
    type DeleteReq = IDReq;
}

impl Into<CreateRoleSchema> for CreateRoleReq {
    fn into(self) -> CreateRoleSchema {
        let permissions = Permission::to_json_value(&self.permissions);
        CreateRoleSchema {
            name: self.name,
            permissions: Some(permissions),
        }
    }
}

impl Into<UpdateRoleSchema> for UpdateRoleReq {
    fn into(self) -> UpdateRoleSchema {
        let permissions = self.permissions.map(|x| Permission::to_json_value(&x));
        UpdateRoleSchema {
            permissions,
            id: self.id,
            name: self.name,
        }
    }
}

impl Into<ListRolePageSchema> for ListPageReq {
    fn into(self) -> ListRolePageSchema {
        ListRolePageSchema {}
    }
}

impl RoleAPI {
    async fn list_permisson() -> ApiJsonResult<ListPermissionResp> {
        let permissions = Permission::all_labels();
        let resp = ListPermissionResp { permissions };
        resp.into_ok_json()
    }
}
