use anyhow::Context;
use log::debug;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub database_uri: String,
    pub phrases_per_round: usize,

    pub input_box_width: usize,
}

impl Config {
    pub fn load<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let config = Config::build(path)?;
        config.validate()?;
        Ok(config)
    }

    fn build<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let builder = config::Config::builder().add_source(config::File::from(path.as_ref()));
        let cfg = builder.build().context("Failed to build configuration")?;
        let config: Config = cfg
            .try_deserialize()
            .context("Failed to deserialize configuration")?;
        debug!("Configuration built");
        Ok(config)
    }

    fn validate(&self) -> anyhow::Result<()> {
        if !Path::new(&self.database_uri).exists() {
            anyhow::bail!("Database file does not exist: {}", self.database_uri);
        }

        if self.phrases_per_round == 0 {
            anyhow::bail!("Phrases per round must be greater than zero.");
        }

        if self.input_box_width < 30 {
            anyhow::bail!("Input box width must be greater than or equal to 30.");
        }

        debug!("Configuration validated");
        Ok(())
    }
}
