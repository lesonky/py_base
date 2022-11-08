use crate::ctx::{build_api_context, Config};
use anyhow::Context;
use axum::extract::Extension;
use axum::Server;

use tower::ServiceBuilder;

pub mod core;
mod files;
mod route;
mod user;
pub use self::core::schemas;

pub async fn serve(config: Config) -> anyhow::Result<()> {
    let addr = format!("{}:{}", &config.host, &config.port);
    let addr = addr.parse()?;
    let ctx = build_api_context(config).await?;

    let layers = ServiceBuilder::new().layer(Extension(ctx));
    let app = route::router().layer(layers);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .context("err running Http server")?;
    Ok(())
}
