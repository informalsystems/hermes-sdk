use core::fmt::Display;

use cgp_core::CanRaiseError;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use hermes_runtime_components::traits::fs::write_file::CanWriteStringToFile;
use hermes_runtime_components::traits::os::exec_command::CanExecCommand;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_test_components::chain::traits::types::wallet::HasWalletType;
use hermes_test_components::chain_driver::traits::fields::chain_home_dir::HasChainHomeDir;
use hermes_test_components::chain_driver::traits::governance::proposal_id::HasProposalIdType;
use hermes_test_components::chain_driver::traits::types::chain::HasChain;

use crate::bootstrap::traits::fields::chain_command_path::HasChainCommandPath;
use crate::chain::types::wallet::CosmosTestWallet;
use crate::chain_driver::traits::deposit_proposal::GovernanceProposalDepositer;
use crate::chain_driver::traits::rpc_port::HasRpcPort;

pub struct DepositGovernanceProposalWithChainCommand;

impl<ChainDriver, Chain, Runtime> GovernanceProposalDepositer<ChainDriver>
    for DepositGovernanceProposalWithChainCommand
where
    ChainDriver: HasRuntime<Runtime = Runtime>
        + HasChain<Chain = Chain>
        + HasProposalIdType
        + HasChainCommandPath
        + HasChainHomeDir
        + HasRpcPort
        + CanRaiseError<Runtime::Error>,
    Runtime: CanExecCommand + CanWriteStringToFile,
    Chain: HasChainId + HasWalletType<Wallet = CosmosTestWallet>,
    ChainDriver::ProposalId: Display,
{
    async fn deposit_proposal(
        chain_driver: &ChainDriver,
        proposal_id: &ChainDriver::ProposalId,
        amount: &str,
        sender: &CosmosTestWallet,
    ) -> Result<String, ChainDriver::Error> {
        let output = chain_driver
            .runtime()
            .exec_command(
                chain_driver.chain_command_path(),
                &[
                    "tx",
                    "gov",
                    "deposit",
                    &proposal_id.to_string(),
                    amount,
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
