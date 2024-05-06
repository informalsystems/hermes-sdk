use core::fmt::Display;

use cgp_core::{Async, CanRaiseError};
use hermes_runtime_components::traits::fs::write_file::CanWriteStringToFile;
use hermes_runtime_components::traits::os::exec_command::CanExecCommand;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_test_components::chain::traits::proposal::types::proposal_id::HasProposalIdType;
use hermes_test_components::chain::traits::proposal::types::proposal_status::{
    HasProposalStatusType, ProvideProposalStatusType,
};
use hermes_test_components::chain_driver::traits::fields::chain_home_dir::HasChainHomeDir;
use serde::Deserialize;

use crate::bootstrap::traits::fields::chain_command_path::HasChainCommandPath;
use crate::chain::types::proposal_status::ProposalStatus;
use crate::chain_driver::traits::proposal_status::GovernanceProposalStatusQuerier;
use crate::chain_driver::traits::rpc_port::HasRpcPort;

pub struct ProvideCosmosProposalStatusType;

impl<ChainDriver> ProvideProposalStatusType<ChainDriver> for ProvideCosmosProposalStatusType
where
    ChainDriver: Async,
{
    type ProposalStatus = ProposalStatus;
}

pub struct QueryGovernanceProposalStatusWithChainCommand;

impl<ChainDriver, Runtime> GovernanceProposalStatusQuerier<ChainDriver>
    for QueryGovernanceProposalStatusWithChainCommand
where
    ChainDriver: HasRuntime<Runtime = Runtime>
        + HasProposalIdType
        + HasProposalStatusType<ProposalStatus = ProposalStatus>
        + HasChainCommandPath
        + HasChainHomeDir
        + HasRpcPort
        + CanRaiseError<Runtime::Error>
        + CanRaiseError<serde_json::Error>,
    Runtime: CanExecCommand + CanWriteStringToFile,
    ChainDriver::ProposalId: Display,
{
    async fn query_proposal_status(
        chain_driver: &ChainDriver,
        proposal_id: &ChainDriver::ProposalId,
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
