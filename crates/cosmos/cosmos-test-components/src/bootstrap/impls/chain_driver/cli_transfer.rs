use alloc::string::String;

use hashbrown::HashMap;
use hermes_core::chain_components::traits::HasChainId;
use hermes_core::runtime_components::traits::{CanExecCommand, HasRuntime};
use hermes_prelude::*;
use hermes_test_components::chain::traits::{TokenCliTransferrer, TokenCliTransferrerComponent};
use hermes_test_components::chain_driver::traits::{
    HasChain, HasChainCommandPath, HasChainHomeDir,
};

use crate::chain_driver::traits::HasRpcPort;

#[cgp_new_provider(TokenCliTransferrerComponent)]
impl<ChainDriver, Runtime> TokenCliTransferrer<ChainDriver> for SendTransferMessageWithCosmosCli
where
    ChainDriver: HasRuntime<Runtime = Runtime>
        + HasChainCommandPath
        + HasChain
        + HasRpcPort
        + HasChainHomeDir
        + CanRaiseAsyncError<Runtime::Error>,
    ChainDriver::Chain: HasChainId,
    Runtime: CanExecCommand + HasAsyncErrorType,
{
    async fn cli_transfer_token(
        chain_driver: &ChainDriver,
        args: HashMap<&str, String>,
    ) -> Result<(), ChainDriver::Error> {
        let runtime = chain_driver.runtime();

        let chain_id = chain_driver.chain().chain_id();
        let rpc_port = chain_driver.rpc_port();

        runtime
            .exec_command(
                chain_driver.chain_command_path(),
                &[
                    "--node",
                    &format!("tcp://localhost:{rpc_port}"),
                    "tx",
                    "ibc-transfer",
                    "transfer",
                    args.get("port_id").unwrap(),
                    args.get("channel_id").unwrap(),
                    args.get("recipient").unwrap(),
                    args.get("amount").unwrap(),
                    "--from",
                    args.get("sender").unwrap(),
                    "--chain-id",
                    chain_id.to_string().as_str(),
                    "--home",
                    &Runtime::file_path_to_string(chain_driver.chain_home_dir()),
                    "--keyring-backend",
                    "test",
                    "--fees",
                    args.get("fees").unwrap(),
                    "--yes",
                ],
            )
            .await
            .map_err(ChainDriver::raise_error)?;

        Ok(())
    }
}
