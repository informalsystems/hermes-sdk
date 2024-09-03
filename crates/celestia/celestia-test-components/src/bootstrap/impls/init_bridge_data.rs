use cgp::core::error::CanRaiseError;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_runtime_components::traits::fs::create_dir::CanCreateDir;
use hermes_runtime_components::traits::fs::file_path::HasFilePathType;
use hermes_runtime_components::traits::os::exec_command::CanExecCommandWithEnvs;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;

use crate::bootstrap::traits::init_bridge_data::BridgeDataInitializer;

pub struct InitCelestiaBridgeData;

impl<Bootstrap, Runtime, Chain> BridgeDataInitializer<Bootstrap> for InitCelestiaBridgeData
where
    Bootstrap:
        HasChainType<Chain = Chain> + HasRuntime<Runtime = Runtime> + CanRaiseError<Runtime::Error>,
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
