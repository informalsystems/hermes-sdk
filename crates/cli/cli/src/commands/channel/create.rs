use oneline_eyre::eyre::Context;
use tracing::info;

use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;
use hermes_cosmos_client_components::types::channel::CosmosInitChannelOptions;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
use hermes_relayer_components::build::traits::components::relay_builder::CanBuildRelay;
use hermes_relayer_components::build::traits::target::relay::RelayAToBTarget;
use hermes_relayer_components::relay::impls::channel::bootstrap::CanBootstrapChannel;

use ibc_relayer::channel::version::Version;
use ibc_relayer_types::core::ics04_channel::channel::Ordering;
use ibc_relayer_types::core::ics24_host::identifier::{ChainId, ClientId, ConnectionId, PortId};

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

impl CommandRunner<CosmosBuilder> for ChannelCreate {
    async fn run(&self, builder: &CosmosBuilder) -> Result<Output> {
        let relay = builder
            .build_relay(
                RelayAToBTarget,
                &self.chain_id_a,
                &self.chain_id_b,
                &self.client_id_a,
                &self.client_id_b,
            )
            .await
            .wrap_err("relayer failed to start")?;

        let options = CosmosInitChannelOptions {
            ordering: self.ordering,
            connection_hops: vec![self.connection_id_a.clone()],
            channel_version: self.version.clone(),
        };

        info!(
            ?options,
            "Creating channel between `{}`:`{}` and `{}`:`{}` on connection `{}`...",
            self.chain_id_a,
            self.client_id_a,
            self.chain_id_b,
            self.client_id_b,
            self.connection_id_a,
        );

        let (channel_id_a, channel_id_b) = relay
            .bootstrap_channel(&self.port_id_a, &self.port_id_b, &options)
            .await
            .wrap_err("failed to create channel; channel handshake failed")?;

        info!(
            %channel_id_a, %channel_id_b,
            "Channel successfully created between `{}` and `{}`",
            self.chain_id_a, self.chain_id_b,
        );

        Ok(Output::success_msg("Done"))
    }
}
