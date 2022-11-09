// common used functions for web api
pub mod crud;
pub mod error;
pub mod resp;
pub mod schemas;

pub use crud::CrudAPI;
pub use error::ApiJsonResult;
pub use resp::IntoOkJson;
