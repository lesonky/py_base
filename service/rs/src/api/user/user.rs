use super::jwt;
use crate::models::user::{EditUserSchema, QueryFilter, User};
use crate::prelude::*;
use axum::body::StreamBody;
use axum::http::{StatusCode};
use tokio_util::io::ReaderStream;

use axum::response::IntoResponse;
use log::debug;

use super::schemas::*;
use axum::extract::Extension;
use axum::extract::{ContentLengthLimit, Multipart};
use axum::routing::{get, post};
use axum::{extract::Query, Json, Router};
use std::path::Path as FsPath;
use tokio::fs;
use uuid::Uuid;

pub fn router() -> Router {
    Router::new()
        .route("/api/user/login", post(login_user))
        .route("/api/user/edit", post(edit_user))
        .route("/api/user/detail", get(detail_user))
        .route("/api/user/list", get(list_user))
        .route("/api/user/avatar", post(upload_avatar))
        .route("/api/user/avatar", get(access_avatar))
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
    resp.into_ok_json()
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
    user.into_ok_json()
}

async fn edit_user(
    Extension(ctx): Extension<ApiContext>,
    Json(req): Json<EditUserSchema>,
) -> ApiJsonResult<User> {
    let id = req.id;
    User::update_one(&ctx.db, req).await?;
    let user = User::find_by_id(&ctx.db, id).await?;
    user.into_ok_json()
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
    resp.into_ok_json()
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
struct UploadAvatarResp {
    url: String,
}

async fn upload_avatar(
    Extension(ctx): Extension<ApiContext>,
    ContentLengthLimit(mut multipart): ContentLengthLimit<
        Multipart,
        {
            1 * 1024 * 1024 /* 1 MB*/
        },
    >,
) {
    if let Some(field) = multipart.next_field().await.unwrap() {
        let file_name = field.file_name().unwrap().to_string();
        let ext = FsPath::new(&file_name)
            .extension()
            .map(|x| x.to_str())
            .flatten()
            .unwrap_or("");

        let file_name = format!("{}.{}", Uuid::new_v4(), ext);
        let data = field.bytes().await.unwrap();
        let avatar_root = FsPath::new(&ctx.config.avatar_root);
        let avatar_path = avatar_root.join(file_name);
        fs::write(avatar_path, data).await.unwrap();
    }
}

#[derive(serde::Deserialize, Debug, Clone)]
struct AvatarName {
    name: String,
}

async fn access_avatar(
    Extension(ctx): Extension<ApiContext>,
    Query(avatar): Query<AvatarName>,
) -> impl IntoResponse {
    let path = ctx.config.avatar_root.join(avatar.name);
    debug!("the avatar path is {:?}", path);
    let file = match tokio::fs::File::open(path).await {
        Ok(file) => file,
        Err(err) => return Err((StatusCode::NOT_FOUND, format!("File not found: {}", err))),
    };
    // convert the `AsyncRead` into a `Stream`
    let stream = ReaderStream::new(file);
    // convert the `Stream` into an `axum::body::HttpBody`
    let body = StreamBody::new(stream);

    //let headers = Headers([
    //    (header::CONTENT_TYPE, "text/toml; charset=utf-8"),
    //    //(
    //    //    header::CONTENT_DISPOSITION,
    //    //    "attachment; filename=\"Cargo.toml\"",
    //    //),
    //]);

    Ok(body)
}
