use core::fmt::Display;

use cgp_core::error::CanRaiseError;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use hermes_runtime_components::traits::fs::write_file::CanWriteStringToFile;
use hermes_runtime_components::traits::os::exec_command::CanExecCommand;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_test_components::chain::traits::proposal::types::proposal_id::HasProposalIdType;
use hermes_test_components::chain::traits::types::wallet::HasWalletType;
use hermes_test_components::chain_driver::traits::fields::chain_home_dir::HasChainHomeDir;
use hermes_test_components::chain_driver::traits::proposal::vote::ProposalVoter;
use hermes_test_components::chain_driver::traits::types::chain::HasChain;

use crate::bootstrap::traits::fields::chain_command_path::HasChainCommandPath;
use crate::chain::types::wallet::CosmosTestWallet;
use crate::chain_driver::traits::rpc_port::HasRpcPort;

pub struct VoteProposalWithChainCommand;

impl<ChainDriver, Chain, Runtime> ProposalVoter<ChainDriver> for VoteProposalWithChainCommand
where
    ChainDriver: HasRuntime<Runtime = Runtime>
        + HasChain<Chain = Chain>
        + HasChainCommandPath
        + HasChainHomeDir
        + HasRpcPort
        + CanRaiseError<Runtime::Error>,
    Runtime: CanExecCommand + CanWriteStringToFile,
    Chain: HasChainId + HasProposalIdType + HasWalletType<Wallet = CosmosTestWallet>,
    Chain::ProposalId: Display,
{
    async fn vote_proposal(
        chain_driver: &ChainDriver,
        proposal_id: &Chain::ProposalId,
        sender: &CosmosTestWallet,
    ) -> Result<(), ChainDriver::Error> {
        chain_driver
            .runtime()
            .exec_command(
                chain_driver.chain_command_path(),
                &[
                    "tx",
                    "gov",
                    "vote",
                    &proposal_id.to_string(),
                    "yes",
                    "--chain-id",
                    &chain_driver.chain().chain_id().to_string(),
                    "--node",
                    &format!("tcp://localhost:{}", chain_driver.rpc_port()),
                    "--home",
                    &Runtime::file_path_to_string(chain_driver.chain_home_dir()),
                    "--from",
                    &sender.id,
                    "--keyring-backend",
                    "test",
                    "-y",
                ],
            )
            .await
            .map_err(ChainDriver::raise_error)?;

        Ok(())
    }
}
