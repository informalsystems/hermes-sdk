use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::{json, Output};
use hermes_cosmos_chain_components::traits::chain_handle::HasBlockingChainHandle;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
use hermes_error::traits::wrap::ErrorWrapper;
use ibc_relayer::chain::counterparty::{channel_connection_client, unreceived_acknowledgements};
use ibc_relayer::path::PathIdentifiers;
use ibc_relayer_types::core::ics04_channel::packet::Sequence;
use ibc_relayer_types::core::ics24_host::identifier::{ChainId, ChannelId, PortId};
use ibc_relayer_types::Height;
use oneline_eyre::eyre::eyre;

use super::util::PacketSequences;
use crate::Result;

#[derive(Debug, clap::Parser)]
pub struct QueryPendingAcks {
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

impl QueryPendingAcks {
    async fn execute(&self, builder: &CosmosBuilder) -> Result<Option<(Vec<Sequence>, Height)>> {
        let port_id = self.port_id.clone();
        let channel_id = self.channel_id.clone();
        let chain = builder.build_chain(&self.chain_id).await?;

        let chan_conn_cli = chain
            .with_blocking_chain_handle(move |handle| {
                let chan_conn_cli = channel_connection_client(&handle, &port_id, &channel_id)
                    .map_err(|e| eyre!("failed to get channel connection and client: {}", e))?;
                Ok(chan_conn_cli)
            })
            .await?;

        let counterparty_chain_id = chan_conn_cli.client.client_state.chain_id();
        let counterparty_chain = builder.build_chain(&counterparty_chain_id.clone()).await?;

        let channel = chan_conn_cli.channel.clone();
        let path_identifiers =
            PathIdentifiers::from_channel_end(channel.clone()).ok_or_else(|| {
                eyre!(
                    "failed to get the path identifiers for channel ({}, {})",
                    channel.channel_id,
                    channel.port_id
                )
            })?;

        unreceived_acknowledgements(&chain.handle, &counterparty_chain.handle, &path_identifiers)
            .wrap_error("failed to get the unreceived acknowledgments")
    }
}

impl CommandRunner<CosmosBuilder> for QueryPendingAcks {
    async fn run(&self, builder: &CosmosBuilder) -> Result<Output> {
        match self.execute(builder).await {
            Err(e) => Ok(Output::error(e)),
            Ok(None) => Ok(Output::success_msg("No unreceived acknowledgements")),
            Ok(Some((sequences, height))) => {
                let packet_sequences = PacketSequences::new(height, sequences);

                if json() {
                    Ok(Output::success(packet_sequences))
                } else {
                    Ok(Output::success(packet_sequences.collated()))
                }
            }
        }
    }
}
