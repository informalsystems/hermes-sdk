use cgp::prelude::*;
use hermes_relayer_components::chain::traits::HasChainIdType;
use hermes_runtime_components::traits::{
    CanCreateDir, CanExecCommandWithEnvs, HasFilePathType, HasRuntime,
};
use hermes_test_components::chain_driver::traits::HasChainType;

use crate::bootstrap::traits::init_bridge_data::{
    BridgeDataInitializer, BridgeDataInitializerComponent,
};

pub struct InitCelestiaBridgeData;

#[cgp_provider(BridgeDataInitializerComponent)]
impl<Bootstrap, Runtime, Chain> BridgeDataInitializer<Bootstrap> for InitCelestiaBridgeData
where
    Bootstrap: HasChainType<Chain = Chain>
        + HasRuntime<Runtime = Runtime>
        + CanRaiseAsyncError<Runtime::Error>,
    Runtime: HasFilePathType + CanExecCommandWithEnvs + CanCreateDir,
    Chain: HasChainIdType,
{
    async fn init_bridge_data(
        bootstrap: &Bootstrap,
        bridge_home_dir: &Runtime::FilePath,
        chain_id: &Chain::ChainId,
    ) -> Result<(), Bootstrap::Error> {
        bootstrap
            .runtime()
            .create_dir(bridge_home_dir)
            .await
            .map_err(Bootstrap::raise_error)?;

        bootstrap
            .runtime()
            .exec_command_with_envs(
                &Runtime::file_path_from_string("celestia"),
                &["bridge", "init", "--p2p.network", &chain_id.to_string()],
                &[("HOME", &Runtime::file_path_to_string(bridge_home_dir))],
            )
            .await
            .map_err(Bootstrap::raise_error)?;

        Ok(())
    }
}
