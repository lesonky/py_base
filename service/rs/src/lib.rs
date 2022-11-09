pub mod api;
pub mod ctx;
pub mod entity;
pub mod models;

use sqlx::MySqlPool as DBPool;

pub use api::core::schemas;
mod prelude {
    pub use super::api::{ApiJsonResult, CrudAPI, IntoOkJson, Result};
    pub use super::ctx::ApiContext;
    pub use super::entity::CrudEntity;
}
