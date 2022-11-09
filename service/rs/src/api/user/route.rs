use super::avatar;
use super::role;
use super::user;
use axum::Router;

pub fn router() -> Router {
    Router::new()
        .merge(user::router())
        .merge(role::router())
        .merge(avatar::router())
}
