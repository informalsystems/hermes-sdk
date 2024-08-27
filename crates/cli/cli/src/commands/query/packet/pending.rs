use core::fmt;

use hermes_cli_components::traits::build::CanLoadBuilder;
use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::{json, Output};
use hermes_cosmos_chain_components::traits::chain_handle::HasBlockingChainHandle;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use ibc_relayer::chain::counterparty::{
    channel_connection_client, channel_on_destination, pending_packet_summary, PendingPackets,
};
use ibc_relayer::chain::requests::Paginate;
use ibc_relayer_types::core::ics24_host::identifier::{ChainId, ChannelId, PortId};
use oneline_eyre::eyre::eyre;
use serde::Serialize;

use super::util::CollatedPendingPackets;
use crate::contexts::app::HermesApp;
use crate::Result;

#[derive(Debug, clap::Parser)]
pub struct QueryPendingPackets {
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

/// A structure to display pending packet commitment sequence IDs
/// at both ends of a channel.
#[derive(Debug, Serialize)]
struct Summary<P> {
    /// Source chain
    src_chain: ChainId,

    /// Destination chain
    dst_chain: ChainId,

    /// The packets sent on the source chain as identified by the command.
    src: P,

    /// The packets sent on the counterparty chain.
    dst: P,
}

impl Summary<PendingPackets> {
    fn collate(self) -> Summary<CollatedPendingPackets> {
        Summary {
            src_chain: self.src_chain,
            dst_chain: self.dst_chain,

            src: CollatedPendingPackets::new(self.src),
            dst: CollatedPendingPackets::new(self.dst),
        }
    }
}

impl fmt::Display for Summary<CollatedPendingPackets> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Summary of pending packets:")?;

        writeln!(f, "Packets pending on source chain ({}):", self.src_chain)?;
        writeln!(f, "  Unreceived packets:")?;
        for seq in &self.src.unreceived_packets {
            writeln!(f, "    {}", seq)?;
        }
        writeln!(f, "  Unreceived acks:")?;
        for seq in &self.src.unreceived_acks {
            writeln!(f, "    {}", seq)?;
        }

        writeln!(
            f,
            "Packets pending on destination chain ({}):",
            self.dst_chain
        )?;
        writeln!(f, "  Unreceived packets:")?;
        for seq in &self.dst.unreceived_packets {
            writeln!(f, "    {}", seq)?;
        }
        writeln!(f, "  Unreceived acks:")?;
        for seq in &self.dst.unreceived_acks {
            writeln!(f, "    {}", seq)?;
        }

        Ok(())
    }
}

impl QueryPendingPackets {
    async fn execute(&self, builder: &CosmosBuilder) -> Result<Summary<PendingPackets>> {
        let chain_id = self.chain_id.clone();
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

        let src_summary = pending_packet_summary(
            &chain.handle,
            &counterparty_chain.handle,
            &chan_conn_cli.channel,
            Paginate::All,
        )
        .map_err(|e| eyre!("failed to get pending packet summary: {}", e))?;

        let counterparty_channel = channel_on_destination(
            &chan_conn_cli.channel,
            &chan_conn_cli.connection,
            &counterparty_chain.handle,
        )
        .map_err(|e| eyre!("failed to get channel on destination: {}", e))?
        .ok_or_else(|| {
            eyre!(
                "missing counterparty channel for ({}, {})",
                chan_conn_cli.channel.channel_id,
                chan_conn_cli.channel.port_id
            )
        })?;

        let dst_summary = pending_packet_summary(
            &counterparty_chain.handle,
            &chain.handle,
            &counterparty_channel,
            Paginate::All,
        )
        .map_err(|e| eyre!("failed to get pending packet summary: {}", e))?;

        Ok(Summary {
            src_chain: chain_id,
            dst_chain: counterparty_chain_id,
            src: src_summary,
            dst: dst_summary,
        })
    }
}

impl CommandRunner<HermesApp> for QueryPendingPackets {
    async fn run(&self, app: &HermesApp) -> Result<Output> {
        let builder = app.load_builder().await?;

        match self.execute(&builder).await {
            Ok(summary) if json() => Ok(Output::success(summary)),
            Ok(summary) => Ok(Output::success_msg(summary.collate().to_string())),
            Err(e) => Ok(Output::error(e)),
        }
    }
}
