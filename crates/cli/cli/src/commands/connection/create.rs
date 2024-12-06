use core::marker::PhantomData;
use std::time::Duration;

use hermes_cli_components::traits::build::CanLoadBuilder;
use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;
use hermes_cosmos_chain_components::types::connection::CosmosInitConnectionOptions;
use hermes_relayer_components::build::traits::builders::relay_builder::CanBuildRelay;
use hermes_relayer_components::multi::types::index::Index;
use hermes_relayer_components::relay::impls::connection::bootstrap::CanBootstrapConnection;
use ibc::core::connection::types::version::Version;
use ibc::core::host::types::identifiers::{ChainId, ClientId};
use oneline_eyre::eyre::eyre;
use tracing::info;

use crate::contexts::app::HermesApp;
use crate::Result;

#[derive(Debug, clap::Parser)]
pub struct ConnectionCreate {
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

impl CommandRunner<HermesApp> for ConnectionCreate {
    async fn run(&self, app: &HermesApp) -> Result<Output> {
        let builder = app.load_builder().await?;

        let relay = builder
            .build_relay(
                PhantomData::<(Index<0>, Index<1>)>,
                &self.chain_id_a,
                &self.chain_id_b,
                &self.client_id_a,
                &self.client_id_b,
            )
            .await
            .map_err(|e| eyre!("Failed to build relay: {e}"))?;

        let options = CosmosInitConnectionOptions {
            delay_period: Duration::from_secs(0),
            connection_version: Version::compatibles().first().unwrap().clone(),
        };

        info!(
            ?options,
            "Creating connection between {}:{} and {}:{}...",
            self.chain_id_a,
            self.client_id_a,
            self.chain_id_b,
            self.client_id_b
        );

        let (connection_id_a, connection_id_b) = relay
            .bootstrap_connection(&options)
            .await
            .map_err(|e| eyre!("Failed to create connection: connection handshake failed: {e}"))?;

        info!(
            %connection_id_a, %connection_id_b,
            "Connection successfully created between {}:{} and {}:{}",
            self.chain_id_a, self.client_id_a, self.chain_id_b, self.client_id_b
        );

        Ok(Output::success_msg("Done"))
    }
}
