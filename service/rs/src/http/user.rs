use super::util::jwt;
use super::{json, ApiJsonResult};
use crate::config::Config;
use crate::models::User;
use axum::extract::Extension;
use axum::routing::post;
use axum::{Json, Router};
use log::debug;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub fn router() -> Router {
    Router::new().route("/api/user/login", post(login_user))
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

async fn login_user(
    Json(req): Json<LoginUserReq>,
    Extension(ref db): Extension<DatabaseConnection>,
    Extension(config): Extension<Config>,
) -> ApiJsonResult<LoginUserResp> {
    debug!("the config is {:?}", config);
    let user = User::find(db, &req.name, &req.password).await?;
    let id = Uuid::new_v4();
    let token = jwt::sign(id, &config.jwt_secret)?;
    let resp = LoginUserResp {
        token: token.to_string(),
    };
    Ok(json(&resp))
}
