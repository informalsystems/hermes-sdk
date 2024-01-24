use cgp_core::prelude::*;
use cgp_core::CanRaiseError;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_test_components::runtime::traits::read_file::CanReadFileAsString;
use hermes_test_components::runtime::traits::types::file_path::HasFilePathType;
use hermes_test_components::runtime::traits::write_file::CanWriteStringToFile;
use serde_json::{Error as JsonError, Value};

use crate::bootstrap::traits::initializers::init_genesis_config::GenesisConfigInitializer;
use crate::bootstrap::traits::modifiers::modify_genesis_config::CanModifyCosmosGenesisConfig;
use crate::bootstrap::traits::types::genesis_config::HasGenesisConfigType;
use crate::bootstrap::types::genesis_config::CosmosGenesisConfig;

/// Parse the generated genesis JSON file, and allow the bootstrap context to modify the genesis config
pub struct UpdateCosmosGenesisConfig;

#[async_trait]
impl<Bootstrap, Runtime> GenesisConfigInitializer<Bootstrap> for UpdateCosmosGenesisConfig
where
    Bootstrap: HasRuntime<Runtime = Runtime>
        + HasGenesisConfigType
        + CanModifyCosmosGenesisConfig
        + CanRaiseError<Runtime::Error>
        + CanRaiseError<JsonError>,
    Runtime: HasFilePathType + CanReadFileAsString + CanWriteStringToFile,
    Bootstrap::GenesisConfig: From<CosmosGenesisConfig>,
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
            .map_err(Bootstrap::raise_error)?;

        let mut config_json: Value =
            serde_json::from_str(&config_string).map_err(Bootstrap::raise_error)?;

        bootstrap.modify_genesis_config(&mut config_json)?;

        let modified_config_string =
            serde_json::to_string_pretty(&config_json).map_err(Bootstrap::raise_error)?;

        runtime
            .write_string_to_file(&genesis_file_path, &modified_config_string)
            .await
            .map_err(Bootstrap::raise_error)?;

        // TODO: generate random denom

        let genesis_config = CosmosGenesisConfig { config_json };

        Ok(genesis_config.into())
    }
}
