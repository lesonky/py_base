use crate::config::Config;
use anyhow::Context;
use axum::extract::Extension;
use axum::{Router, Server};
use sea_orm::DatabaseConnection;
use tower::ServiceBuilder;
use std::sync::Arc;

use sqlx::mysql::MySqlPool;
use sqlx::mysql::MySqlPoolOptions;

pub mod error;
pub mod models;
mod resp;
mod test;
mod user;
mod util;

use error::{ApiJsonResult, Error, Result};
use resp::json;

#[derive(Clone)]
pub struct ApiContext {
    config: Arc<Config>,
    db: MySqlPool,
}

fn router() -> Router {
    Router::new()
        .nest("/api/test", test::router())
        .merge(user::router())
}

pub async fn serve(config: Config, db: DatabaseConnection) -> anyhow::Result<()> {
    let addr = format!("{}:{}", &config.host, &config.port);
    let addr = addr.parse()?;
    let mysql_db = MySqlPoolOptions::new().max_connections(50).connect(&config.database_url).await.context("could not connect to database url")?;
    let ctx = ApiContext {
        config: Arc::new(config.clone()),
        db: mysql_db
    };

    let layers = ServiceBuilder::new()
        .layer(Extension(db))
        .layer(Extension(config))
        .layer(Extension(ctx));
    let app = router().layer(layers);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .context("err running Http server")?;
    Ok(())
}
