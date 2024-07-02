use cgp_core::run::CanRun;
use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
use hermes_relayer_components::build::traits::components::birelay_builder::CanBuildBiRelay;
use ibc_relayer_types::core::ics24_host::identifier::{ChainId, ClientId};
use oneline_eyre::eyre::eyre;
use tracing::info;

use crate::Result;

#[derive(Debug, clap::Parser)]
pub struct Start {
    /// Identifier of chain A
    #[clap(
        long = "chain-a",
        required = true,
        value_name = "CHAIN_ID_A",
        help_heading = "REQUIRED"
    )]
    chain_id_a: ChainId,

    /// Identifier of client A
    #[clap(
        long = "client-a",
        required = true,
        value_name = "CLIENT_ID_A",
        help_heading = "REQUIRED"
    )]
    client_id_a: ClientId,

    /// Identifier of chain B
    #[clap(
        long = "chain-b",
        required = true,
        value_name = "CHAIN_ID_B",
        help_heading = "REQUIRED"
    )]
    chain_id_b: ChainId,

    /// Identifier of client B
    #[clap(
        long = "client-b",
        required = true,
        value_name = "CLIENT_ID_B",
        help_heading = "REQUIRED"
    )]
    client_id_b: ClientId,
}

impl CommandRunner<CosmosBuilder> for Start {
    async fn run(&self, builder: &CosmosBuilder) -> Result<Output> {
        info!("Starting relayer...");

        let birelay = builder
            .build_birelay(
                &self.chain_id_a,
                &self.chain_id_b,
                &self.client_id_a,
                &self.client_id_b,
            )
            .await
            .map_err(|e| eyre!("Relayer failed to start: {e}"))?;

        info!(
            "Relaying between {} and {}...",
            self.chain_id_a, self.chain_id_b
        );

        birelay
            .run()
            .await
            .map_err(|e| eyre!("Relayed exited because of error: {e}"))?;

        Ok(Output::success_msg("Relayer exited successfully."))
    }
}
