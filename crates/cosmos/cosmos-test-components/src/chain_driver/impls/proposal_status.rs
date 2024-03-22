use cgp_core::CanRaiseError;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_test_components::chain_driver::traits::fields::chain_home_dir::HasChainHomeDir;
use hermes_test_components::runtime::traits::exec_command::CanExecCommand;
use hermes_test_components::runtime::traits::write_file::CanWriteStringToFile;
use serde::Deserialize;

use crate::bootstrap::traits::fields::chain_command_path::HasChainCommandPath;
use crate::chain_driver::traits::proposal_status::GovernanceProposalStatusQuerier;
use crate::chain_driver::traits::rpc_port::HasRpcPort;

pub struct QueryGovernanceProposalStatusWithChainCommand;

#[derive(Deserialize)]
pub struct Response {
    pub status: String,
}

impl<ChainDriver, Runtime> GovernanceProposalStatusQuerier<ChainDriver>
    for QueryGovernanceProposalStatusWithChainCommand
where
    ChainDriver: HasRuntime<Runtime = Runtime>
        + HasChainCommandPath
        + HasChainHomeDir
        + HasRpcPort
        + CanRaiseError<Runtime::Error>
        + CanRaiseError<serde_json::Error>,
    Runtime: CanExecCommand + CanWriteStringToFile,
{
    async fn query_proposal_status(
        chain_driver: &ChainDriver,
        proposal_id: &str,
    ) -> Result<String, ChainDriver::Error> {
        let output = chain_driver
            .runtime()
            .exec_command(
                chain_driver.chain_command_path(),
                &[
                    "query",
                    "gov",
                    "proposal",
                    proposal_id,
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
