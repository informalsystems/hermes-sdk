use core::fmt::Display;

use hermes_core::relayer_components::chain::traits::HasChainIdType;
use hermes_core::runtime_components::traits::{CanCreateDir, HasRuntime};
use hermes_core::test_components::chain_driver::traits::HasChainType;
use hermes_prelude::*;

use crate::bootstrap::traits::{
    ChainHomeDirInitializer, ChainHomeDirInitializerComponent, HasChainStoreDir,
};

pub struct CreateChainHomeDirFromTestDir;

#[cgp_provider(ChainHomeDirInitializerComponent)]
impl<Bootstrap, Runtime, Chain> ChainHomeDirInitializer<Bootstrap> for CreateChainHomeDirFromTestDir
where
    Bootstrap: HasChainType<Chain = Chain>
        + HasRuntime<Runtime = Runtime>
        + CanRaiseAsyncError<Runtime::Error>
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
