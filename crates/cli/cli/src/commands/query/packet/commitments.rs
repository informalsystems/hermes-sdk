use oneline_eyre::eyre::Context;

use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::{json, Output};
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_relayer_components::chain::traits::queries::packet_commitments::CanQueryPacketCommitments;

use ibc_relayer_types::core::ics24_host::identifier::{ChainId, ChannelId, PortId};

use crate::commands::query::packet::util::PacketSequences;
use crate::Result;

#[derive(Debug, clap::Parser)]
pub struct QueryPacketCommitments {
    /// Identifier of the host chain
    #[clap(
        long = "chain",
        required = true,
        value_name = "CHAIN_ID",
        help_heading = "REQUIRED"
    )]
    chain_id: ChainId,

    /// Identifier of the port
    #[clap(
        long = "port",
        required = true,
        value_name = "PORT_ID",
        help_heading = "REQUIRED"
    )]
    port_id: PortId,

    /// Identifier of the channel
    #[clap(
        long = "channel",
        required = true,
        value_name = "CHANNEL_ID",
        help_heading = "REQUIRED"
    )]
    channel_id: ChannelId,
}

impl CommandRunner<CosmosBuilder> for QueryPacketCommitments {
    async fn run(&self, builder: &CosmosBuilder) -> Result<Output> {
        let chain = builder
            .build_chain(&self.chain_id)
            .await
            .wrap_err_with(|| format!("failed to bu;ild chain `{}`", self.chain_id))?;

        let (sequences, height) =
            <CosmosChain as CanQueryPacketCommitments<CosmosChain>>::query_packet_commitments(
                &chain,
                &self.channel_id,
                &self.port_id,
            )
            .await
            .wrap_err("`query packet commitments` command failed")?;

        let packet_sequences = PacketSequences::new(height, sequences);

        if json() {
            Ok(Output::success(packet_sequences))
        } else {
            Ok(Output::success(packet_sequences.collated()))
        }
    }
}
