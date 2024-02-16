use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;

use hermes_relayer_components::build::traits::components::birelay_builder::CanBuildBiRelay;
use hermes_relayer_components::relay::traits::packet_clearer::CanClearPackets;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;
use ibc_relayer_types::core::ics24_host::identifier::ChannelId;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;
use ibc_relayer_types::core::ics24_host::identifier::PortId;
use tracing::error;

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

impl CommandRunner<CosmosBuilder> for PacketsClear {
    async fn run(&self, builder: &CosmosBuilder) -> Result<Output> {
        let relayer = builder
            .build_birelay(
                &self.chain_id,
                &self.counterparty_chain_id,
                &self.client_id,
                &self.counterparty_client_id, // nothing to pass here
            )
            .await?;

        let task_a_to_b = relayer.relay_a_to_b.clear_packets(
            &self.channel_id,
            &self.port_id,
            &self.counterparty_channel_id,
            &self.counterparty_port_id,
        );

        let task_b_to_a = relayer.relay_b_to_a.clear_packets(
            &self.counterparty_channel_id,
            &self.counterparty_port_id,
            &self.channel_id,
            &self.port_id,
        );

        let (result_a_to_b, result_b_to_a) = futures::join!(task_a_to_b, task_b_to_a);

        if let Err(e) = &result_a_to_b {
            error!(
                "failed to clear packets from `{}` to `{}`: {e}",
                self.chain_id, self.counterparty_port_id
            );
        }

        if let Err(e) = &result_b_to_a {
            error!(
                "failed to clear packets from `{}` to `{}`: {e}",
                self.counterparty_port_id, self.chain_id
            );
        }

        if result_a_to_b.is_err() || result_b_to_a.is_err() {
            Ok(Output::error("failed to clear packets"))
        } else {
            Ok(Output::success_msg("successfully cleared packets"))
        }
    }
}
