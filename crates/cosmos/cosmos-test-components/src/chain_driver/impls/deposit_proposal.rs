use cgp_core::CanRaiseError;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_test_components::chain_driver::traits::fields::chain_home_dir::HasChainHomeDir;
use hermes_test_components::chain_driver::traits::types::chain::HasChain;
use hermes_test_components::runtime::traits::exec_command::CanExecCommand;
use hermes_test_components::runtime::traits::write_file::CanWriteStringToFile;

use crate::bootstrap::traits::fields::chain_command_path::HasChainCommandPath;
use crate::chain_driver::traits::deposit_proposal::GovernanceProposalDepositer;
use crate::chain_driver::traits::rpc_port::HasRpcPort;

pub struct DepositGovernanceProposalWithChainCommand;

impl<ChainDriver, Chain, Runtime> GovernanceProposalDepositer<ChainDriver>
    for DepositGovernanceProposalWithChainCommand
where
    ChainDriver: HasRuntime<Runtime = Runtime>
        + HasChain<Chain = Chain>
        + CanRaiseError<Runtime::Error>
        + HasChainCommandPath
        + HasChainHomeDir
        + HasRpcPort,
    Runtime: CanExecCommand + CanWriteStringToFile,
    Chain: HasChainId,
{
    async fn deposit_proposal(
        chain_driver: &ChainDriver,
        proposal_id: &str,
        amount: &str,
        sender: &str,
    ) -> Result<String, ChainDriver::Error> {
        let output = chain_driver
            .runtime()
            .exec_command(
                chain_driver.chain_command_path(),
                &[
                    "tx",
                    "gov",
                    "deposit",
                    proposal_id,
                    amount,
                    "--chain-id",
                    &chain_driver.chain().chain_id().to_string(),
                    "--node",
                    &format!("tcp://localhost:{}", chain_driver.rpc_port()),
                    "--home",
                    &Runtime::file_path_to_string(chain_driver.chain_home_dir()),
                    "--from",
                    sender,
                    "--keyring-backend",
                    "test",
                    "--gas",
                    "auto",
                    "-y",
                ],
            )
            .await
            .map_err(ChainDriver::raise_error)?;

        Ok(output.stdout)
    }
}
