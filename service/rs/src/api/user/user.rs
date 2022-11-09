use super::jwt;
use super::schemas::*;
use crate::models::user::{
    CreateUserSchema, QueryFilter, UpdateUserSchema, User, UserListPageSchema,
};
use crate::prelude::*;
use crate::schemas::*;
use axum::extract::Extension;
use axum::routing::{get, post};
use axum::{Json, Router};

//TODO: should add middleare to check permission of user table
pub fn router() -> Router {
    Router::new()
        .route("/api/user/create", get(UserAPI::create))
        .route("/api/user/delete", get(UserAPI::delete))
        .route("/api/user/list", get(UserAPI::list_page))
        .route("/api/user/detail", get(UserAPI::detail))
        .route("/api/user/edit", post(UserAPI::update))
        .route("/api/user/login", post(UserAPI::login))
}
pub struct UserAPI {}

impl CrudAPI for UserAPI {
    type EntityModel = User;
    type CreateReq = CreateUserSchema;
    type UpdateReq = UpdateUserSchema;
    type ListPageReq = UserListPageSchema;
    type DetailReq = IDReq;
    type DeleteReq = IDReq;
}

impl UserAPI {
    async fn login(
        Extension(ctx): Extension<ApiContext>,
        Json(req): Json<LoginUserReq>,
    ) -> ApiJsonResult<LoginUserResp> {
        let filters = QueryFilter {
            id: Some(1),
            name: Some(&req.name),
            hashed_passwd: Some(&req.password),
            ..Default::default()
        };
        let user = User::find_one(&ctx.db, &filters).await?;
        let token = jwt::sign(user.account_id, &ctx.config.jwt_secret)?;
        let resp = LoginUserResp {
            token: token.to_string(),
        };
        resp.into_ok_json()
    }
}
