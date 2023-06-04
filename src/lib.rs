pub mod config;
pub mod db;
pub mod err;
pub mod form;
pub mod handler;
pub mod model;
pub mod view;

pub type Result<T> = std::result::Result<T, crate::err::Error>;
