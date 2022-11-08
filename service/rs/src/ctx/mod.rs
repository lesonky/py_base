pub mod config;
use std::sync::Arc;

use anyhow::Context;

pub use config::Config;
use sqlx::mysql::MySqlPool;
use sqlx::mysql::MySqlPoolOptions;

#[derive(Clone)]
pub struct ApiContext {
    pub config: Arc<Config>,
    pub db: MySqlPool,
}

pub async fn build_api_context(config: Config) -> anyhow::Result<ApiContext> {
    let mysql_db = MySqlPoolOptions::new()
        .max_connections(50)
        .connect(&config.database_url)
        .await
        .context("could not connect to database url")?;
    let ctx = ApiContext {
        config: Arc::new(config.clone()),
        db: mysql_db,
    };
    Ok(ctx)
}
