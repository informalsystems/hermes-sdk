use cgp_core::prelude::*;
use eyre::Report;

use crate::traits::bootstrap::read_file::CanReadFileAsString;
use crate::traits::bootstrap::write_file::CanWriteStringToFile;
use crate::traits::file_path::HasFilePathType;
use crate::traits::init_genesis_file::GenesisFileInitializer;
use crate::traits::update_genesis_config::CanUpdateGenesisJsonConfig;

/// Parse the generated genesis JSON file, and allow the bootstrap context to modify the genesis config
pub struct UpdateCosmosGenesisConfig;

#[async_trait]
impl<Bootstrap> GenesisFileInitializer<Bootstrap> for UpdateCosmosGenesisConfig
where
    Bootstrap: HasFilePathType
        + HasErrorType
        + CanUpdateGenesisJsonConfig
        + CanReadFileAsString
        + CanWriteStringToFile,
    Bootstrap::Error: From<Report>,
{
    async fn init_genesis_file(
        bootstrap: &Bootstrap,
        chain_home_dir: &Bootstrap::FilePath,
    ) -> Result<(), Bootstrap::Error> {
        let genesis_file_path = Bootstrap::join_file_path(
            chain_home_dir,
            &Bootstrap::file_path_from_string("config/genesis.json"),
        );

        let config_string = bootstrap.read_file_as_string(&genesis_file_path).await?;

        let mut config = serde_json::from_str(&config_string).map_err(Report::from)?;

        bootstrap.update_genesis_json_config(&mut config)?;

        let modified_config_string = serde_json::to_string_pretty(&config).map_err(Report::from)?;

        bootstrap
            .write_string_to_file(&genesis_file_path, &modified_config_string)
            .await?;

        Ok(())
    }
}
