use crate::commands::query::packet::util::PacketSeqs;
use crate::Result;
use hermes_cli_framework::command::Runnable;
use hermes_cli_framework::output::{json, Output};
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_relayer_components::chain::traits::queries::packet_commitments::CanQueryPacketCommitments;
use ibc_relayer_types::core::ics24_host::identifier::{ChainId, ChannelId, PortId};

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

impl Runnable for QueryPacketCommitments {
    async fn run(&self, builder: CosmosBuilder) -> Result<Output> {
        let port_id = self.port_id.clone();
        let channel_id = self.channel_id.clone();
        let chain = builder.build_chain(&self.chain_id).await?;

        let (sequences, height) =
            <CosmosChain as CanQueryPacketCommitments<CosmosChain>>::query_packet_commitments(
                &chain,
                &channel_id,
                &port_id,
            )
            .await?;

        let packet_sequences = PacketSeqs {
            height,
            seqs: sequences,
        };

        if json() {
            Ok(Output::success(packet_sequences))
        } else {
            Ok(Output::success(packet_sequences.collated()))
        }
    }
}
