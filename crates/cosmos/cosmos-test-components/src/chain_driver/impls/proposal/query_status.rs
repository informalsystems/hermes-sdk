use core::fmt::Display;

use cgp_core::CanRaiseError;
use hermes_runtime_components::traits::fs::write_file::CanWriteStringToFile;
use hermes_runtime_components::traits::os::exec_command::CanExecCommand;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_test_components::chain::traits::proposal::types::proposal_id::HasProposalIdType;
use hermes_test_components::chain::traits::proposal::types::proposal_status::HasProposalStatusType;
use hermes_test_components::chain_driver::traits::fields::chain_home_dir::HasChainHomeDir;
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;
use serde::Deserialize;

use crate::bootstrap::traits::fields::chain_command_path::HasChainCommandPath;
use crate::chain::types::proposal_status::ProposalStatus;
use crate::chain_driver::traits::rpc_port::HasRpcPort;
use hermes_test_components::chain_driver::traits::proposal::query_status::ProposalStatusQuerier;

pub struct QueryProposalStatusWithChainCommand;

impl<ChainDriver, Chain, Runtime> ProposalStatusQuerier<ChainDriver>
    for QueryProposalStatusWithChainCommand
where
    ChainDriver: HasRuntime<Runtime = Runtime>
        + HasChainType<Chain = Chain>
        + HasChainCommandPath
        + HasChainHomeDir
        + HasRpcPort
        + CanRaiseError<Runtime::Error>
        + CanRaiseError<serde_json::Error>,
    Runtime: CanExecCommand + CanWriteStringToFile,
    Chain: HasProposalIdType + HasProposalStatusType<ProposalStatus = ProposalStatus>,
    Chain::ProposalId: Display,
{
    async fn query_proposal_status(
        chain_driver: &ChainDriver,
        proposal_id: &Chain::ProposalId,
    ) -> Result<ProposalStatus, ChainDriver::Error> {
        let output = chain_driver
            .runtime()
            .exec_command(
                chain_driver.chain_command_path(),
                &[
                    "query",
                    "gov",
                    "proposal",
                    &proposal_id.to_string(),
                    "--node",
                    &format!("tcp://localhost:{}", chain_driver.rpc_port()),
                    "--home",
                    &Runtime::file_path_to_string(chain_driver.chain_home_dir()),
                    "--output",
                    "json",
                ],
            )
            .await
            .map_err(ChainDriver::raise_error)?;

        let response: Response =
            serde_json::from_str(&output.stdout).map_err(ChainDriver::raise_error)?;

        Ok(response.status)
    }
}

#[derive(Deserialize)]
struct Response {
    status: ProposalStatus,
}
