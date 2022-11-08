// common used functions for web api
pub mod error;
pub mod resp;
pub mod schemas;

pub use error::ApiJsonResult;
pub use resp::IntoOkJson;
