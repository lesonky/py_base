pub mod api;
pub mod config;
pub mod entity;
pub mod models;

use api::error;
use sqlx::MySqlPool as DBPool;
