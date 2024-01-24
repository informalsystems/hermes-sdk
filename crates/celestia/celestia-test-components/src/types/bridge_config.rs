use std::path::PathBuf;

pub struct CelestiaBridgeConfig {
    pub config: toml::Value,
    pub bridge_home_dir: PathBuf,
    pub bridge_config_dir: PathBuf,
}
