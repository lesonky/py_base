use log::info;
use sea_orm::Database;
use std::env;

use webace_http_base::{api, config::Config};

fn parse_config_from_env() -> Config {
    //TODO: use clap to parse config
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET is not in .env file");
    let passwd_salt = env::var("PASSWD_SALT").expect("JWT_SECRET is not in .env file");

    info!("config: DATABASE_URL {}", &db_url);
    info!("http server run on {}:{}", host, port);
    let config = Config {
        database_url: db_url.clone(),
        host,
        port,
        jwt_secret,
        passwd_salt,
    };
    config
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();
    let config = parse_config_from_env();
    let db = Database::connect(&config.database_url)
        .await
        .expect("Database connection failed");

    api::serve(config, db).await?;
    Ok(())
}
