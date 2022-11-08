use axum::routing::post;
use axum::Router;

pub fn router() -> Router {
    Router::new()
        .route(
            "/api/files/upload_session/start",
            post(upload_session_start),
        )
        .route(
            "/api/files/upload_session/append",
            post(upload_session_append),
        )
        .route(
            "/api/files/upload_session/finish",
            post(upload_session_finish),
        )
}

async fn upload_session_start() {
    unimplemented!()
}

async fn upload_session_append() {
    unimplemented!()
}

async fn upload_session_finish() {
    unimplemented!()
}
