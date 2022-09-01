use axum::{http::StatusCode, Json};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Resp<T> {
    pub code: u16,
    pub success: bool,
    pub message: Option<String>,
    pub data: Option<T>,
}

impl<T> Resp<T>
where
    T: Serialize + DeserializeOwned + Clone,
{
    pub fn from_result(arg: &T) -> Self {
        Self {
            code: 200,
            success: true,
            message: Some("ok".to_string()),
            data: Some(arg.clone()),
        }
    }
    pub fn from_error_info(code: StatusCode, info: &str) -> Self {
        Self {
            code: code.as_u16(),
            success: false,
            message: Some(info.to_string()),
            data: None,
        }
    }

    pub fn bad_param(message: Option<&str>) -> Self {
        Self {
            code: StatusCode::BAD_REQUEST.as_u16(),
            success: false,
            message: message.map(|x| x.to_string()),
            data: None,
        }
    }
}

pub fn json<T>(arg: &T) -> Json<Resp<T>>
where
    T: Serialize + DeserializeOwned + Clone,
{
    Json(Resp::from_result(arg))
}

pub fn json_bad_param(message: Option<&str>) -> Json<Resp<()>> {
    Json(Resp::bad_param(message))
}
