use super::util::jwt;
use crate::api::{json, ApiContext, ApiJsonResult};
use crate::models::user::{EditUserSchema, QueryFilter, User};

use axum::extract::Extension;
use axum::routing::{get, post};
use axum::{extract::Query, Json, Router};

pub fn router() -> Router {
    Router::new()
        .route("/api/user/login", post(login_user))
        .route("/api/user/edit", post(edit_user))
        .route("/api/user/detail", get(detail_user))
        .route("/api/user/list", get(list_user))
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

async fn login_user(
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

async fn detail_user(
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

async fn edit_user(
    Extension(ctx): Extension<ApiContext>,
    Json(req): Json<EditUserSchema>,
) -> ApiJsonResult<User> {
    let id = req.id;
    User::update_one(&ctx.db, req).await?;
    let user = User::find_by_id(&ctx.db, id).await?;
    return Ok(json(&user));
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
#[warn(non_snake_case)]
struct ListPageReq {
    name_icontain: Option<String>,
    page_num: Option<u64>,
    page_size: Option<u64>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct ListPageResp {
    items: Vec<User>,
    total: u64,
}

async fn list_user(
    Extension(ctx): Extension<ApiContext>,
    Query(req): Query<ListPageReq>,
) -> ApiJsonResult<ListPageResp> {
    let filter = QueryFilter {
        page_num: Some(req.page_num.unwrap_or(1)),
        page_size: Some(req.page_size.unwrap_or(10)),
        ..Default::default()
    };
    let (items, total) = User::find_page(&ctx.db, &filter).await?;
    let resp = ListPageResp { items, total };
    return Ok(json(&resp));
}
