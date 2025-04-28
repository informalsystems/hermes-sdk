use hermes_prelude::*;
use futures::stream::{self, StreamExt};
use hermes_cli_components::traits::CanLoadBuilder;
use hermes_cli_components::traits::CommandRunnerComponent;
use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;
use hermes_core::relayer_components::build::traits::builders::birelay_builder::CanBuildBiRelay;
use hermes_core::relayer_components::relay::traits::CanClearPackets;
use hermes_cosmos_core::ibc::core::host::types::identifiers::{ChainId, ChannelId, ClientId, PortId};

use crate::contexts::HermesApp;
use crate::Result;

#[derive(Debug, clap::Parser)]
pub struct PacketsClear {
    #[clap(
        long = "chain",
        required = true,
        value_name = "CHAIN_ID",
        help_heading = "REQUIRED",
        help = "Identifier of the chain to query"
    )]
    chain_id: ChainId,

    #[clap(
        long = "port",
        required = true,
        value_name = "PORT_ID",
        help_heading = "REQUIRED",
        help = "Identifier of the port"
    )]
    port_id: PortId,

    #[clap(
        long = "channel",
        alias = "chan",
        required = true,
        value_name = "CHANNEL_ID",
        help_heading = "REQUIRED",
        help = "Identifier of the channel"
    )]
    channel_id: ChannelId,

    #[clap(
        long = "client",
        required = true,
        value_name = "CLIENT_ID",
        help_heading = "REQUIRED",
        help = "Identifier of the client"
    )]
    client_id: ClientId,

    #[clap(
        long = "counterparty-chain",
        required = true,
        value_name = "COUNTERPARTY_CHAIN_ID",
        help_heading = "REQUIRED",
        help = "Identifier of the counterparty chain to query"
    )]
    counterparty_chain_id: ChainId,

    #[clap(
        long = "counterparty-client",
        required = true,
        value_name = "COUNTERPARTY_CLIENT_ID",
        help_heading = "REQUIRED",
        help = "Identifier of the counterparty client"
    )]
    counterparty_client_id: ClientId,

    #[clap(
        long = "counterparty-port",
        required = true,
        value_name = "COUNTERPARTY_PORT_ID",
        help_heading = "REQUIRED",
        help = "Identifier of the counterparty port"
    )]
    counterparty_port_id: PortId,

    #[clap(
        long = "counterparty-channel",
        required = true,
        value_name = "COUNTERPARTY_CHANNEL_ID",
        help_heading = "REQUIRED",
        help = "Identifier of the counterparty channel"
    )]
    counterparty_channel_id: ChannelId,
}

#[cgp_provider(CommandRunnerComponent)]
impl CommandRunner<HermesApp> for PacketsClear {
    async fn run(&self, app: &HermesApp) -> Result<Output> {
        let builder = app.load_builder().await?;

        let relayer = builder
            .build_birelay(
                &self.chain_id,
                &self.counterparty_chain_id,
                &self.client_id,
                &self.counterparty_client_id, // nothing to pass here
            )
            .await?;

        stream::iter(vec![
            relayer.relay_a_to_b.clear_packets(
                &self.channel_id,
                &self.port_id,
                &self.counterparty_channel_id,
                &self.counterparty_port_id,
            ),
            relayer.relay_b_to_a.clear_packets(
                &self.counterparty_channel_id,
                &self.counterparty_port_id,
                &self.channel_id,
                &self.port_id,
            ),
        ])
        .for_each_concurrent(None, |x| async {
            let _ = x.await;
        })
        .await;

        Ok(Output::success("Packet clear"))
    }
}
