#[derive(Debug, Default, Clone)]
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: String,
    pub jwt_secret: String,
    pub passwd_salt: String,
}
