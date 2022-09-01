use crate::config::Config;
use anyhow::Context;
use axum::extract::Extension;
use axum::{Router, Server};
use sea_orm::DatabaseConnection;
use tower::ServiceBuilder;

pub mod error;
pub mod models;
mod resp;
mod test;
mod user;
mod util;

use error::{ApiJsonResult, Error, Result};
use resp::json;

fn router() -> Router {
    Router::new()
        .nest("/api/test", test::router())
        .merge(user::router())
}

pub async fn serve(config: Config, db: DatabaseConnection) -> anyhow::Result<()> {
    let addr = format!("{}:{}", &config.host, &config.port);
    let addr = addr.parse()?;
    let layer = ServiceBuilder::new()
        .layer(Extension(db))
        .layer(Extension(config));
    let app = router().layer(layer);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .context("err running Http server")?;
    Ok(())
}
