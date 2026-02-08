use anyhow::Context;
use log::{debug, trace};
use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::types::LogLevel;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub db_conn_string: String,

    pub log_level: LogLevel,
    pub log_dir_uri: Option<String>,

    pub input_box_width: usize,
    pub phrases_per_round: usize,
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
        self.sanitize_string(&self.db_conn_string)?;

        if let Some(ref log_dir_uri) = self.log_dir_uri {
            self.sanitize_string(log_dir_uri)?;
        }

        if self.phrases_per_round == 0 {
            anyhow::bail!("Phrases per round must be greater than zero.");
        }

        if self.input_box_width < 30 {
            anyhow::bail!("Input box width must be greater than or equal to 30.");
        }

        trace!("Configuration parsed");
        Ok(())
    }

    fn sanitize_string(&self, value: &str) -> anyhow::Result<()> {
        // TODO implement sanitization logic

        trace!("String sanitized: {}", value);
        Ok(())
    }
}
