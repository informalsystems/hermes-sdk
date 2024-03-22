use cgp_core::CanRaiseError;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use hermes_runtime_components::traits::fs::write_file::CanWriteStringToFile;
use hermes_runtime_components::traits::os::exec_command::CanExecCommand;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_test_components::chain_driver::traits::fields::chain_home_dir::HasChainHomeDir;
use hermes_test_components::chain_driver::traits::types::chain::HasChain;

use crate::bootstrap::traits::fields::chain_command_path::HasChainCommandPath;
use crate::chain_driver::traits::rpc_port::HasRpcPort;
use crate::chain_driver::traits::vote_proposal::GovernanceProposalVoter;

pub struct VoteGovernanceProposalWithChainCommand;

impl<ChainDriver, Chain, Runtime> GovernanceProposalVoter<ChainDriver>
    for VoteGovernanceProposalWithChainCommand
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
    async fn vote_proposal(
        chain_driver: &ChainDriver,
        proposal_id: &str,
        sender: &str,
    ) -> Result<String, ChainDriver::Error> {
        let output = chain_driver
            .runtime()
            .exec_command(
                chain_driver.chain_command_path(),
                &[
                    "tx",
                    "gov",
                    "vote",
                    proposal_id,
                    "yes",
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
                    "-y",
                ],
            )
            .await
            .map_err(ChainDriver::raise_error)?;

        Ok(output.stdout)
    }
}
