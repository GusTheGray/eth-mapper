use config::{Config, ConfigError, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AlchemyWebsocket {
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct DbSettings {
    pub url: String,
    pub user: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub alchemy_websocket: AlchemyWebsocket,
    pub db_settings: DbSettings,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let s = Config::builder()
            .add_source(File::with_name("config/default"))
            .add_source(File::with_name("config/local").required(false))
            .build()?;

        s.try_deserialize()
    }
}
