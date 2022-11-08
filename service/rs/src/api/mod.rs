use crate::ctx::{build_api_context, Config};
use anyhow::Context;
use axum::extract::Extension;
use axum::Server;

use tower::ServiceBuilder;

pub mod core;
mod files;
mod route;
mod user;

pub async fn serve(config: Config) -> anyhow::Result<()> {
    let addr = format!("{}:{}", &config.host, &config.port);
    let addr = addr.parse()?;
    let ctx = build_api_context(config).await?;
    //let mysql_db = MySqlPoolOptions::new()
    //    .max_connections(50)
    //    .connect(&config.database_url)
    //    .await
    //    .context("could not connect to database url")?;
    //let ctx = ApiContext {
    //    config: Arc::new(config.clone()),
    //    db: mysql_db,
    //};

    let layers = ServiceBuilder::new().layer(Extension(ctx));
    let app = route::router().layer(layers);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .context("err running Http server")?;
    Ok(())
}
