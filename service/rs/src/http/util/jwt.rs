use crate::http::Result;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: i64,
    pub iat: i64,
}

impl Claims {
    pub fn new(id: Uuid) -> Self {
        let iat = Utc::now();
        let exp = iat + Duration::hours(24);
        Self {
            sub: id,
            iat: iat.timestamp(),
            exp: exp.timestamp(),
        }
    }
}

pub fn sign(id: Uuid, jwt_secret: &str) -> Result<String> {
    let claim = Claims::new(id);
    let header = Header::default();
    let key = EncodingKey::from_secret(jwt_secret.as_bytes());
    let token = jsonwebtoken::encode(&header, &claim, &key).unwrap();
    Ok(token)
}

pub fn verify(token: &str, jwt_secret: &str) -> Result<Claims> {
    let key = DecodingKey::from_secret(jwt_secret.as_bytes());
    let validation = Validation::default();
    let claims = jsonwebtoken::decode(token, &key, &validation)
        .map(|data| data.claims)
        .unwrap();
    Ok(claims)
}
