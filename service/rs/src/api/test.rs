use crate::api::resp::json;
use crate::error::ApiJsonResult;
use crate::error::{Error, Result};
use axum::routing::get;
use axum::Router;

pub fn router() -> Router {
    Router::new()
        .route("/ping", get(ping))
        .route("/json", get(test))
        .route("/error", get(test_error))
}

async fn ping() -> &'static str {
    "pong"
}

async fn test() -> ApiJsonResult<String> {
    let msg = "ok".to_string();
    Ok(json(&msg))
}

fn call_resp_error() -> Result<()> {
    Err(Error::Unauthorized {})
}

async fn test_error() -> ApiJsonResult<String> {
    let _ret = call_resp_error()?;
    let msg = "ok".to_string();
    Ok(json(&msg))
}
