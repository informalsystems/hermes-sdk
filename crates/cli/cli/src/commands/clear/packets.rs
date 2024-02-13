use oneline_eyre::eyre::Context;

use hermes_cli_framework::command::Runnable;
use hermes_cli_framework::output::Output;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
use hermes_cosmos_relayer::contexts::relay::CosmosRelay;
use hermes_relayer_components::build::traits::components::relay_builder::CanBuildRelay;
use hermes_relayer_components::build::traits::target::relay::RelayAToBTarget;
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainHeight;
use hermes_relayer_components::relay::traits::packet_clearer::CanClearPackets;
use hermes_relayer_components::relay::traits::target::DestinationTarget;
use hermes_relayer_components::relay::traits::update_client_message_builder::CanSendUpdateClientMessage;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;
use ibc_relayer_types::core::ics24_host::identifier::ChannelId;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;
use ibc_relayer_types::core::ics24_host::identifier::PortId;

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

impl Runnable for PacketsClear {
    async fn run(&self, builder: CosmosBuilder) -> Result<Output> {
        let chain = builder.build_chain(&self.chain_id).await?;

        let relayer = builder
            .build_relay(
                RelayAToBTarget,
                &self.chain_id,
                &self.counterparty_chain_id,
                &self.client_id,
                &self.counterparty_client_id, // nothing to pass here
            )
            .await?;

        let target_height = chain.query_chain_height().await?;

        relayer
            .send_update_client_messages(DestinationTarget, &target_height)
            .await
            .wrap_err("Failed to send update client message")?;

        <CosmosRelay as CanClearPackets>::clear_packets(
            &relayer,
            &self.channel_id,
            &self.port_id,
            &self.counterparty_channel_id,
            &self.counterparty_port_id,
        )
        .await?;

        Ok(Output::success("Packet clear"))
    }
}
