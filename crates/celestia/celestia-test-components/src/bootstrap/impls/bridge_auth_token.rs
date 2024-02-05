use cgp_core::CanRaiseError;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;
use hermes_test_components::runtime::traits::exec_command::CanExecCommandWithEnvs;

use crate::bootstrap::traits::bridge_auth_token::BridgeAuthTokenGenerator;
use crate::bootstrap::traits::types::bridge_driver::HasBridgeDriverType;
use crate::bridge_driver::traits::bridge_auth_token::HasBridgeAuthTokenType;

pub struct GenerateBridgeJwtToken;

impl<Bootstrap, Runtime, Chain, BridgeDriver> BridgeAuthTokenGenerator<Bootstrap>
    for GenerateBridgeJwtToken
where
    Bootstrap: HasRuntime<Runtime = Runtime>
        + HasChainType<Chain = Chain>
        + HasBridgeDriverType<BridgeDriver = BridgeDriver>
        + CanRaiseError<Runtime::Error>,
    Runtime: CanExecCommandWithEnvs,
    Chain: HasChainIdType,
    BridgeDriver: HasBridgeAuthTokenType<BridgeAuthToken = String>,
{
    async fn generate_bridge_auth_token(
        bootstrap: &Bootstrap,
        bridge_home_dir: &Runtime::FilePath,
        chain_id: &Chain::ChainId,
    ) -> Result<BridgeDriver::BridgeAuthToken, Bootstrap::Error> {
        let output = bootstrap
            .runtime()
            .exec_command_with_envs(
                &Runtime::file_path_from_string("celestia"),
                &[
                    "bridge",
                    "auth",
                    "admin",
                    "--p2p.network",
                    &chain_id.to_string(),
                ],
                &[("HOME", &Runtime::file_path_to_string(bridge_home_dir))],
            )
            .await
            .map_err(Bootstrap::raise_error)?;

        Ok(output.stdout.trim().into())
    }
}
