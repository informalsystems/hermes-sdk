use core::fmt::Display;

use cgp_core::prelude::*;
use cgp_core::CanRaiseError;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;
use hermes_test_components::runtime::traits::create_dir::CanCreateDir;

use crate::bootstrap::traits::fields::chain_store_dir::HasChainStoreDir;
use crate::bootstrap::traits::initializers::init_chain_home_dir::ChainHomeDirInitializer;

pub struct CreateChainHomeDirFromTestDir;

#[async_trait]
impl<Bootstrap, Runtime, Chain> ChainHomeDirInitializer<Bootstrap> for CreateChainHomeDirFromTestDir
where
    Bootstrap: HasChainType<Chain = Chain>
        + HasRuntime<Runtime = Runtime>
        + CanRaiseError<Runtime::Error>
        + HasChainStoreDir,
    Runtime: CanCreateDir,
    Chain: HasChainIdType,
    Chain::ChainId: Display,
{
    async fn init_chain_home_dir(
        bootstrap: &Bootstrap,
        chain_id: &Chain::ChainId,
    ) -> Result<Runtime::FilePath, Bootstrap::Error> {
        let chain_store_dir = bootstrap.chain_store_dir();
        let chain_home_dir = Runtime::join_file_path(
            chain_store_dir,
            &Runtime::file_path_from_string(&chain_id.to_string()),
        );

        bootstrap
            .runtime()
            .create_dir(&chain_home_dir)
            .await
            .map_err(Bootstrap::raise_error)?;

        Ok(chain_home_dir)
    }
}
