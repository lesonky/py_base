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

#[derive(Deserialize, Debug, Clone)]
pub struct CreateRoleReq {
    pub name: String,
    pub permissions: Vec<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DeleteRoleReq {
    pub id: i64,
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateRoleReq {
    pub id: u64,
    pub name: Option<String>,
    pub permissions: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ListPermissionResp {
    pub permissions: Vec<String>,
}
