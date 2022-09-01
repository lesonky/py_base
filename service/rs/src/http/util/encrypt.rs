use bcrypt::DEFAULT_COST;

use crate::error::Result;

// consume password value to make it unusable
pub async fn hash_password(password: String) -> Result<String> {
    bcrypt::hash(password, DEFAULT_COST)
}

pub async fn verify_password(password: String, hash: String) -> Result<bool> {
    bcrypt::verify(password, &hash)
}
