use anyhow::{Context, Result};
use dotenvy::dotenv;
use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct Config {
    pub environment: Environment,
    #[serde(default = "default_port")]
    pub port: u16,
}

const fn default_port() -> u16 {
    8000
}

#[derive(Deserialize)]
pub(crate) enum Environment {
    Prod,
    Dev,
}

impl Config {
    pub fn load() -> Result<Config> {
        let _ = dotenv();
        envy::from_env::<Config>().context("Failed to load environment variables")
    }
}
