use core::marker::PhantomData;

use hermes_cli_components::traits::build::CanLoadBuilder;
use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;
use hermes_cosmos_chain_components::types::channel::CosmosInitChannelOptions;
use hermes_relayer_components::build::traits::builders::relay_builder::CanBuildRelay;
use hermes_relayer_components::multi::types::index::Index;
use hermes_relayer_components::relay::impls::channel::bootstrap::CanBootstrapChannel;
use ibc_relayer::channel::version::Version;
use ibc_relayer_types::core::ics04_channel::channel::Ordering;
use ibc_relayer_types::core::ics24_host::identifier::{ChainId, ClientId, ConnectionId, PortId};
use oneline_eyre::eyre::eyre;
use tracing::info;

use crate::contexts::app::HermesApp;
use crate::Result;

#[derive(Debug, clap::Parser)]
pub struct ChannelCreate {
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

    /// Identifier of the connection on A
    #[clap(
        long = "connection-a",
        required = true,
        value_name = "CONNECTION_ID_A",
        help_heading = "REQUIRED"
    )]
    connection_id_a: ConnectionId,

    /// Port identifier on chain A
    #[clap(
        long = "port-a",
        value_name = "PORT_ID_A",
        default_value_t = PortId::transfer(),
    )]
    port_id_a: PortId,

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

    /// Port identifier on chain B
    #[clap(
        long = "port-b",
        value_name = "PORT_ID_B",
        default_value_t = PortId::transfer(),
    )]
    port_id_b: PortId,

    /// Ordering of the channel
    #[clap(
        long = "ordering",
        value_name = "ORDERING",
        default_value_t = Ordering::Unordered,
    )]
    ordering: Ordering,

    /// Version of the channel
    #[clap(
        long = "version",
        value_name = "VERSION",
        default_value_t = Version::ics20(),
    )]
    version: Version,
}

impl CommandRunner<HermesApp> for ChannelCreate {
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

        let options = CosmosInitChannelOptions {
            ordering: self.ordering,
            connection_hops: vec![self.connection_id_a.clone()],
            channel_version: self.version.clone(),
        };

        info!(
            ?options,
            "Creating channel between {}:{} and {}:{} on connection {}...",
            self.chain_id_a,
            self.client_id_a,
            self.chain_id_b,
            self.client_id_b,
            self.connection_id_a,
        );

        let (channel_id_a, channel_id_b) = relay
            .bootstrap_channel(&self.port_id_a, &self.port_id_b, &options)
            .await
            .map_err(|e| eyre!("Failed to create channel: channel handshake failed: {e}"))?;

        info!(
            %channel_id_a, %channel_id_b,
            "Channel successfully created between {} and {}",
            self.chain_id_a, self.chain_id_b,
        );

        Ok(Output::success_msg("Done"))
    }
}
