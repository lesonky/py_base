pub mod api;
pub mod ctx;
pub mod entity;
pub mod models;

use sqlx::MySqlPool as DBPool;
mod prelude {
    pub use super::api::core::error::Result;
    pub use super::api::core::ApiJsonResult;
    pub use super::api::core::IntoOkJson;
    pub use super::ctx::ApiContext;
}
