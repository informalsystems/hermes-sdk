use cgp_core::prelude::*;
use eyre::Report;
use ibc_relayer_components::runtime::traits::runtime::HasRuntime;
use serde_json::Value;

use crate::traits::initializers::init_genesis_config::GenesisConfigInitializer;
use crate::traits::modifiers::modify_genesis_config::CanModifyCosmosGenesisConfig;
use crate::traits::runtime::read_file::CanReadFileAsString;
use crate::traits::runtime::types::file_path::HasFilePathType;
use crate::traits::runtime::write_file::CanWriteStringToFile;
use crate::traits::types::genesis_config::HasGenesisConfigType;

/// Parse the generated genesis JSON file, and allow the bootstrap context to modify the genesis config
pub struct UpdateCosmosGenesisConfig;

#[async_trait]
impl<Bootstrap, Runtime> GenesisConfigInitializer<Bootstrap> for UpdateCosmosGenesisConfig
where
    Bootstrap: HasRuntime<Runtime = Runtime>
        + HasErrorType
        + HasGenesisConfigType
        + CanModifyCosmosGenesisConfig,
    Runtime: HasFilePathType + CanReadFileAsString + CanWriteStringToFile,
    Bootstrap::Error: From<Report>,
    Bootstrap::GenesisConfig: From<Value>,
{
    async fn init_genesis_config(
        bootstrap: &Bootstrap,
        chain_home_dir: &Runtime::FilePath,
    ) -> Result<Bootstrap::GenesisConfig, Bootstrap::Error> {
        let runtime = bootstrap.runtime();

        let genesis_file_path = Runtime::join_file_path(
            chain_home_dir,
            &Runtime::file_path_from_string("config/genesis.json"),
        );

        let config_string = runtime
            .read_file_as_string(&genesis_file_path)
            .await
            .map_err(Bootstrap::runtime_error)?;

        let mut config: Value = serde_json::from_str(&config_string).map_err(Report::from)?;

        bootstrap.modify_genesis_config(&mut config)?;

        let modified_config_string = serde_json::to_string_pretty(&config).map_err(Report::from)?;

        runtime
            .write_string_to_file(&genesis_file_path, &modified_config_string)
            .await
            .map_err(Bootstrap::runtime_error)?;

        Ok(config.into())
    }
}
