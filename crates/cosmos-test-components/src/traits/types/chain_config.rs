use cgp_core::prelude::*;

use crate::traits::types::file_path::HasFilePathType;

pub trait HasChainConfigType: Async {
    type ChainConfig: Async;
}

pub trait CanParseChainConfig: HasChainConfigType + HasErrorType {
    fn parse_chain_config(config_string: &str) -> Result<Self::ChainConfig, Self::Error>;

    fn serialize_chain_config(config: &Self::ChainConfig) -> Result<String, Self::Error>;
}

pub trait HasChainConfigFile: HasFilePathType {
    fn chain_config_file_path(&self, chain_home_dir: &Self::FilePath) -> Self::FilePath;
}
