use crate::api::Resp;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;

pub type Result<T, E = Error> = std::result::Result<T, E>;
pub type ApiJsonResult<T, E = Error> = Result<Json<Resp<T>>, E>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("authentication required")]
    Unauthorized,

    #[error("user may not perform the action")]
    Forbidden,

    #[error("request path not found")]
    NotFound,

    #[error(transparent)]
    SeaOrmDbError(#[from] sea_orm::DbErr),

    #[error(transparent)]
    SqlxError(#[from] sqlx::Error),

    #[error("row not found in db")]
    RowNotFound,
}

impl Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Forbidden => StatusCode::FORBIDDEN,
            Self::NotFound => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn into_resp<T>(self) -> Resp<T> {
        match self {
            Self::Unauthorized => Resp {
                code: 401,
                success: false,
                message: Some("un authentication".to_string()),
                data: None,
            },
            _ => Resp {
                code: 500,
                success: false,
                message: Some(self.to_string()),
                data: None,
            },
        }
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let status_code = self.status_code();
        let resp = self.into_resp::<()>();
        (status_code, Json(resp)).into_response()
    }
}
