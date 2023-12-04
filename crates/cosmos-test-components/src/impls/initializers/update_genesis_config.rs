use cgp_core::prelude::*;
use eyre::Report;
use serde_json::Value;

use crate::traits::initializers::init_genesis_config::GenesisConfigInitializer;
use crate::traits::io::read_file::CanReadFileAsString;
use crate::traits::io::write_file::CanWriteStringToFile;
use crate::traits::modifiers::modify_genesis_config::CanModifyCosmosGenesisConfig;
use crate::traits::types::genesis_config::HasGenesisConfigType;
use crate::traits::types::io::file_path::HasFilePathType;

/// Parse the generated genesis JSON file, and allow the bootstrap context to modify the genesis config
pub struct UpdateCosmosGenesisConfig;

#[async_trait]
impl<Bootstrap> GenesisConfigInitializer<Bootstrap> for UpdateCosmosGenesisConfig
where
    Bootstrap: HasFilePathType
        + HasErrorType
        + HasGenesisConfigType
        + CanModifyCosmosGenesisConfig
        + CanReadFileAsString
        + CanWriteStringToFile,
    Bootstrap::Error: From<Report>,
    Bootstrap::GenesisConfig: From<Value>,
{
    async fn init_genesis_config(
        bootstrap: &Bootstrap,
        chain_home_dir: &Bootstrap::FilePath,
    ) -> Result<Bootstrap::GenesisConfig, Bootstrap::Error> {
        let genesis_file_path = Bootstrap::join_file_path(
            chain_home_dir,
            &Bootstrap::file_path_from_string("config/genesis.json"),
        );

        let config_string = bootstrap.read_file_as_string(&genesis_file_path).await?;

        let mut config: Value = serde_json::from_str(&config_string).map_err(Report::from)?;

        bootstrap.modify_genesis_config(&mut config)?;

        let modified_config_string = serde_json::to_string_pretty(&config).map_err(Report::from)?;

        bootstrap
            .write_string_to_file(&genesis_file_path, &modified_config_string)
            .await?;

        Ok(config.into())
    }
}
