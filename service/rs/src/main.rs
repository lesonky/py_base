use log::info;
use std::env;
use std::path::PathBuf;

use webace_http_base::{api, ctx::Config};

fn parse_config_from_env() -> Config {
    //TODO: use clap to parse config
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET is not in .env file");
    let passwd_salt = env::var("PASSWD_SALT").expect("JWT_SECRET is not in .env file");
    let avatar_root_s = env::var("AVATAR_ROOT").expect("AVATAR_ROOT is not set in .env file");
    let mut avatar_root = PathBuf::new();
    avatar_root.push(avatar_root_s);

    info!("config: DATABASE_URL {}", &db_url);
    info!("http server run on {}:{}", host, port);
    let config = Config {
        database_url: db_url.clone(),
        host,
        port,
        jwt_secret,
        passwd_salt,
        avatar_root,
    };
    config
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();
    let config = parse_config_from_env();
    api::serve(config).await?;
    Ok(())
}
