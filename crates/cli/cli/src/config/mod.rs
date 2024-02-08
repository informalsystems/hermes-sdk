use std::ops::Deref;

use hermes_cli_framework::config::Config;
use ibc_relayer::config::load as load_config;
use ibc_relayer::config::Config as RelayerConfig;

pub struct HermesConfig {
    pub config: RelayerConfig,
}

impl Deref for HermesConfig {
    type Target = RelayerConfig;

    fn deref(&self) -> &Self::Target {
        &self.config
    }
}

impl Config for HermesConfig {
    fn load_from_path(
        path: impl AsRef<std::path::Path>,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let config = load_config(path)?;
        Ok(Self { config })
    }
}
