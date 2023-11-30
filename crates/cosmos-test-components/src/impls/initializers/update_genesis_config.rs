use cgp_core::prelude::*;
use eyre::Report;

use crate::traits::initializers::init_genesis_config::GenesisConfigInitializer;
use crate::traits::io::read_file::CanReadFileAsString;
use crate::traits::io::write_file::CanWriteStringToFile;
use crate::traits::modifiers::modify_genesis_config::CanModifyGenesisConfig;
use crate::traits::types::file_path::HasFilePathType;
use crate::traits::types::genesis_config::{CanParseGenesisConfig, HasGenesisConfigFile};

/// Parse the generated genesis JSON file, and allow the bootstrap context to modify the genesis config
pub struct UpdateCosmosGenesisConfig;

#[async_trait]
impl<Bootstrap> GenesisConfigInitializer<Bootstrap> for UpdateCosmosGenesisConfig
where
    Bootstrap: HasFilePathType
        + HasErrorType
        + HasGenesisConfigFile
        + CanParseGenesisConfig
        + CanModifyGenesisConfig
        + CanReadFileAsString
        + CanWriteStringToFile,
    Bootstrap::Error: From<Report>,
{
    async fn init_genesis_config(
        bootstrap: &Bootstrap,
        chain_home_dir: &Bootstrap::FilePath,
    ) -> Result<Bootstrap::GenesisConfig, Bootstrap::Error> {
        let genesis_file_path = bootstrap.genesis_config_file_path(chain_home_dir);

        let config_string = bootstrap.read_file_as_string(&genesis_file_path).await?;

        let mut config = Bootstrap::parse_genesis_config(&config_string)?;

        bootstrap.modify_genesis_config(&mut config)?;

        let modified_config_string = Bootstrap::serialize_genesis_config(&config)?;

        bootstrap
            .write_string_to_file(&genesis_file_path, &modified_config_string)
            .await?;

        Ok(config)
    }
}
