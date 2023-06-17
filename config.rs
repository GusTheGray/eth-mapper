use config::{Config, ConfigError, Environment, File};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct AlchemyWebsocket {
    url: String,
}


#[derive(Debug, Deserialize)]
struct Settings {
    alchemy_websocket: AlchemyWebsocket,
}

impl Settings {
    pub fn new() -> Result<Self, ConfigError> {
        let mut s = Config::new();
        s.merge(File::with_name("config/default"))?;
        s.merge(File::with_name("config/local").required(false))?;
        s.merge(Environment::with_prefix("app"))?;
        s.try_into()
    }
}