use std::collections::HashMap;

use cgp_core::CanRaiseError;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_test_components::chain_driver::traits::fields::chain_home_dir::HasChainHomeDir;
use hermes_test_components::runtime::traits::exec_command::CanExecCommand;
use hermes_test_components::runtime::traits::write_file::CanWriteStringToFile;

use crate::bootstrap::traits::fields::chain_command_path::HasChainCommandPath;
use crate::chain_driver::traits::proposal_status::GovernanceProposalStatusQuerier;
use crate::chain_driver::traits::rpc_port::HasRpcPort;

pub struct QueryGovernanceProposalStatusWithChainCommand;

impl<ChainDriver, Runtime> GovernanceProposalStatusQuerier<ChainDriver>
    for QueryGovernanceProposalStatusWithChainCommand
where
    ChainDriver: HasRuntime<Runtime = Runtime>
        + CanRaiseError<Runtime::Error>
        + HasChainCommandPath
        + HasChainHomeDir
        + HasRpcPort,
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

        let json: HashMap<String, serde_json::Value> =
            serde_json::from_str(&output.stdout).unwrap();

        let status_str = json.get("status").unwrap().as_str().unwrap();

        Ok(status_str.to_owned())
    }
}
