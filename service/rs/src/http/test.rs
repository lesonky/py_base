use crate::http::resp::json;
use crate::http::Error;
use crate::http::Result;
use axum::routing::get;
use axum::Router;

use super::ApiJsonResult;

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
