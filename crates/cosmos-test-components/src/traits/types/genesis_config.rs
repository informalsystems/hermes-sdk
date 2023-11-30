use cgp_core::prelude::*;

use crate::traits::types::file_path::HasFilePathType;

pub trait HasGenesisConfigType: Async {
    type GenesisConfig: Async;
}

pub trait CanParseGenesisConfig: HasGenesisConfigType + HasErrorType {
    fn parse_genesis_config(config_string: &str) -> Result<Self::GenesisConfig, Self::Error>;

    fn serialize_genesis_config(config: &Self::GenesisConfig) -> Result<String, Self::Error>;
}

pub trait HasGenesisConfigFile: HasFilePathType {
    fn genesis_config_file_path(&self, chain_home_dir: &Self::FilePath) -> Self::FilePath;
}
