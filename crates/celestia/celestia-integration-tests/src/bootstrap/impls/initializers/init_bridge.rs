use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;
use hermes_test_components::runtime::traits::exec_command::CanExecCommand;
use hermes_test_components::runtime::traits::types::file_path::HasFilePathType;
use hermes_cosmos_test_components::bootstrap::traits::fields::chain_command_path::HasChainCommandPath;
use hermes_cosmos_test_components::bootstrap::traits::initializers::init_chain_data::ChainDataInitializer;

pub struct InitCelestiaBridge;

#[async_trait]
impl<Bootstrap, Runtime, Chain> ChainDataInitializer<Bootstrap> for InitCelestiaBridge
where
    Bootstrap: HasChainType<Chain = Chain> + HasRuntime<Runtime = Runtime>,
    Runtime: HasFilePathType + CanExecCommand,
    Chain: HasChainIdType,
{
    async fn init_chain_data(
        bootstrap: &Bootstrap,
        chain_home_dir: &Runtime::FilePath,
        chain_id: &Chain::ChainId,
    ) -> Result<(), Bootstrap::Error> {
        let chain_id = chain_id.to_string();

        bootstrap
            .runtime()
            .exec_command(
                &Runtime::file_path_from_string("celestia"),
                &[
                    "bridge",
                    "init",
                    "--p2p.network",
                    &chain_id,
                ],
            )
            .await
            .map_err(Bootstrap::raise_error)?;

        Ok(())
    }
}
