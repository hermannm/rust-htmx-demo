use anyhow::{Context, Result};
use dotenvy::dotenv;
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct Config {
    #[serde(default = "default_env")]
    pub environment: Environment,
    #[serde(default = "default_port")]
    pub port: u16,
}

#[derive(Deserialize, Debug)]
pub(crate) enum Environment {
    Prod,
    Dev,
}

const fn default_env() -> Environment {
    Environment::Dev
}

const fn default_port() -> u16 {
    8000
}

impl Config {
    pub fn load() -> Result<Config> {
        let _ = dotenv();
        envy::from_env::<Config>().context("Failed to load environment variables")
    }
}
