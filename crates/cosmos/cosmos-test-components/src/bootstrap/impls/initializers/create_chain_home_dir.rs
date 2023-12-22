use core::fmt::Display;
use std::io::Error as IoError;
use std::path::Path;

use cgp_core::prelude::*;
use cgp_core::CanRaiseError;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_test_components::bootstrap::traits::types::chain::HasChainType;
use hermes_test_components::runtime::traits::types::file_path::HasFilePathType;
use tokio::fs::create_dir_all;

use crate::bootstrap::traits::fields::test_dir::HasTestDir;
use crate::bootstrap::traits::initializers::init_chain_home_dir::ChainHomeDirInitializer;

pub struct CreateChainHomeDirFromTestDir;

#[async_trait]
impl<Bootstrap, Runtime, Chain> ChainHomeDirInitializer<Bootstrap> for CreateChainHomeDirFromTestDir
where
    Bootstrap:
        HasChainType<Chain = Chain> + HasRuntime<Runtime = Runtime> + HasErrorType + HasTestDir,
    Runtime: HasFilePathType + HasErrorType,
    Chain: HasChainIdType,
    Chain::ChainId: Display,
    Runtime::FilePath: AsRef<Path>,
    Bootstrap: CanRaiseError<IoError>,
{
    async fn init_chain_home_dir(
        bootstrap: &Bootstrap,
        chain_id: &Chain::ChainId,
    ) -> Result<Runtime::FilePath, Bootstrap::Error> {
        let test_dir = bootstrap.test_dir();
        let chain_home_dir = Runtime::join_file_path(
            test_dir,
            &Runtime::file_path_from_string(&chain_id.to_string()),
        );

        create_dir_all(&chain_home_dir)
            .await
            .map_err(Bootstrap::raise_error)?;

        Ok(chain_home_dir)
    }
}
