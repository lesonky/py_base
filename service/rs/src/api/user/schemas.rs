use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct LoginUserReq {
    pub name: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginUserResp {
    pub token: String,
}

#[derive(Deserialize, Debug)]
pub struct IdReq {
    pub id: i64,
}
