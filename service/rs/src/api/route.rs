use super::user;
use axum::Router;

pub fn router() -> Router {
    Router::new().merge(user::router())
}
