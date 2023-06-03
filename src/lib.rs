pub mod model;
pub mod config;
pub mod err;

pub type Result<T> = std::result::Result<T, crate::err::Error>;