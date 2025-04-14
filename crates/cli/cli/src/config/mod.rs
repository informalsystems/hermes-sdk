use std::fs::File;
use std::io::Read;
use std::ops::Deref;
use std::path::Path;

use hermes_cli_framework::config::Config;
use hermes_cosmos_chain_components::impls::RelayerConfig;

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
        let config = load_config(&path)?;
        Ok(Self { config })
    }
}

fn load_config(path: &impl AsRef<std::path::Path>) -> Result<RelayerConfig, String> {
    let canonical_path = path
        .as_ref()
        .canonicalize()
        .map_err(|e| format!("failed to canonicalize config path. Cause: {e}"))?;
    load_toml_file(canonical_path.as_path())
}

fn load_toml_file(path: &Path) -> Result<RelayerConfig, String> {
    let mut file =
        File::open(path).map_err(|e| format!("failed to open config file. Cause: {e}"))?;

    let mut toml_string = String::new();
    file.read_to_string(&mut toml_string)
        .map_err(|e| format!("failed to read toml file from string. Cause: {e}"))?;
    load_toml(toml_string)
}

fn load_toml(toml_string: String) -> Result<RelayerConfig, String> {
    toml::from_str(toml_string.as_ref())
        .map_err(|e| format!("failed to convert String to toml. Cause: {e}"))
}
