use anyhow::Context;
use log::{debug, trace};
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
        let mut config = Config::build(path)?;
        config.parse()?;

        debug!("Configuration loaded: {:?}", config);
        Ok(config)
    }

    fn build<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let builder = config::Config::builder().add_source(config::File::from(path.as_ref()));
        let cfg = builder.build().context("Failed to build configuration")?;
        let config: Config = cfg
            .try_deserialize()
            .context("Failed to deserialize configuration")?;

        trace!("Configuration built");
        Ok(config)
    }

    fn parse(&mut self) -> anyhow::Result<()> {
        self.database_uri = Self::parse_uri(&self.database_uri)?;

        if self.phrases_per_round == 0 {
            anyhow::bail!("Phrases per round must be greater than zero.");
        }

        if self.input_box_width < 30 {
            anyhow::bail!("Input box width must be greater than or equal to 30.");
        }

        trace!("Configuration parsed");
        Ok(())
    }

    fn parse_uri(uri: &str) -> anyhow::Result<String> {
        let path = uri
            .strip_prefix("file://")
            .context("Failed to parse database URI")?;
        if !Path::new(path).exists() {
            anyhow::bail!("Database file does not exist: {}", path);
        }

        trace!("URI parsed: {path}");
        Ok(path.to_string())
    }
}
