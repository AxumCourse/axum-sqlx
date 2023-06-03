use serde::Deserialize;

use crate::{err::Error, Result};

#[derive(Deserialize)]
pub struct MySQLConfig {
    pub dsn: String,
    pub maxcons: u32,
}

#[derive(Deserialize)]
pub struct WebConfig {
    pub addr: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub mysql: MySQLConfig,
    pub web: WebConfig,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        config::Config::builder()
            .add_source(config::Environment::default())
            .build()
            .map_err(Error::from)?
            .try_deserialize()
            .map_err(Error::from)
    }
}
