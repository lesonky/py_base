use super::util::jwt;
use crate::api::{json, ApiContext, ApiJsonResult};
use crate::models::user::{EditUserSchema, QueryFilter, User};

use axum::extract::Extension;
use axum::routing::{get, post};
use axum::{extract::Query, Json, Router};

pub fn router() -> Router {
    Router::new()
        .route("/api/user/login", post(user_login))
        .route("/api/user/edit", post(user_edit))
        .route("/api/user/detail", get(user_detail))
}

#[derive(serde::Deserialize, Debug)]
struct LoginUserReq {
    name: String,
    password: String,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct LoginUserResp {
    token: String,
}

#[derive(serde::Deserialize, Debug)]
struct IdReq {
    id: i64,
}

async fn user_login(
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
    Ok(json(&resp))
}

async fn user_detail(
    Extension(ctx): Extension<ApiContext>,
    Query(req): Query<IdReq>,
) -> ApiJsonResult<User> {
    let user = User::find_one(
        &ctx.db,
        &QueryFilter {
            id: Some(req.id),
            ..Default::default()
        },
    )
    .await?;
    Ok(json(&user))
}

async fn user_edit(
    Extension(_ctx): Extension<ApiContext>,
    Json(_req): Json<EditUserSchema>,
) -> ApiJsonResult<User> {
    unimplemented!()
}
