use axum::body::StreamBody;
use axum::extract::{ContentLengthLimit, Multipart};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::routing::{get, post};
use axum::{extract::Query, Router};
use log::debug;
use std::path::Path as FsPath;
use tokio::fs;
use tokio_util::io::ReaderStream;
use uuid::Uuid;

use crate::prelude::*;
use axum::extract::Extension;
pub fn router() -> Router {
    Router::new()
        .route("/api/user/avatar", post(upload_avatar))
        .route("/api/user/avatar", get(access_avatar))
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
