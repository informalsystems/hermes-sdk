use std::path::PathBuf;

use hermes_core::runtime_components::traits::CanExecCommand;
use hermes_core::test_components::test_case::traits::node::{
    FullNodeHalter, FullNodeHalterComponent,
};
use hermes_error::HermesError;
use hermes_prelude::*;

use crate::contexts::CosmosChainDriver;

#[cgp_new_provider(FullNodeHalterComponent)]
impl FullNodeHalter<CosmosChainDriver> for HaltCosmosFullNode {
    async fn halt_full_node(chain_driver: &CosmosChainDriver) -> Result<(), HermesError> {
        let runtime = chain_driver.chain.runtime.clone();
        let node_pid = chain_driver
            .chain_processes
            .first()
            .expect("Failed to retrieve Chain Driver A chain process")
            .id()
            .expect("failed to retrieve Chain Driver A chain process ID");

        // Stop full node
        // `kill` is used here instead of `Child::kill()` as the `kill()` method requires
        // the child process to be mutable.
        runtime
            .exec_command(
                &PathBuf::from("kill".to_string()),
                &["-s", "KILL", &node_pid.to_string()],
            )
            .await?;

        Ok(())
    }
}
