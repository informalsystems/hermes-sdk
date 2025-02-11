use core::fmt;

use cgp::prelude::*;
use hermes_chain_components::traits::queries::chain_status::CanQueryChainHeight;
use hermes_chain_components::traits::queries::packet_acknowledgements::CanQueryPacketAcknowledgements;
use hermes_chain_components::traits::queries::unreceived_acks_sequences::CanQueryUnreceivedAcksSequences;
use hermes_cli_components::traits::build::CanLoadBuilder;
use hermes_cli_components::traits::command::CommandRunnerComponent;
use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::{json, Output};
use hermes_cosmos_chain_components::traits::abci_query::CanQueryAbci;
use hermes_cosmos_chain_components::types::tendermint::TendermintClientState;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_encoding_components::traits::convert::CanConvert;
use hermes_encoding_components::traits::has_encoding::HasDefaultEncoding;
use hermes_protobuf_encoding_components::types::any::Any;
use hermes_relayer_components::chain::traits::queries::packet_commitments::CanQueryPacketCommitments;
use hermes_relayer_components::chain::traits::queries::unreceived_packet_sequences::CanQueryUnreceivedPacketSequences;
use ibc::clients::tendermint::types::TENDERMINT_CLIENT_STATE_TYPE_URL;
use ibc::core::channel::types::channel::ChannelEnd;
use ibc::core::connection::types::ConnectionEnd;
use ibc::core::host::types::identifiers::{ChainId, ChannelId, PortId};
use ibc::cosmos_host::IBC_QUERY_PATH;
use ibc::primitives::proto::Protobuf;
use oneline_eyre::eyre::eyre;
use serde::Serialize;

use crate::commands::query::packet::util::{CollatedPendingPackets, PendingPackets};
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

        let latest_height = chain.query_chain_height().await?;

        // channel end query path
        let channel_end_path = format!("channelEnds/ports/{port_id}/channels/{channel_id}");

        let channel_end_bytes = chain
            .query_abci(IBC_QUERY_PATH, channel_end_path.as_bytes(), &latest_height)
            .await?;

        let channel_end = ChannelEnd::decode_vec(&channel_end_bytes)?;

        let counterparty_channel_id = channel_end.counterparty().channel_id().ok_or_else(|| {
            eyre!(
                "missing counterparty channel ID for channel `{}`",
                channel_id
            )
        })?;
        let counterparty_port_id = channel_end.counterparty().port_id();

        let connection_id = channel_end
            .connection_hops
            .first()
            .ok_or_else(|| eyre!("missing connection ID for channel `{}`", channel_id))?;

        // connection end query path
        let connection_path = format!("connections/{connection_id}");

        let connnection_end_bytes = chain
            .query_abci(IBC_QUERY_PATH, connection_path.as_bytes(), &latest_height)
            .await?;

        let connection_end = ConnectionEnd::decode_vec(&connnection_end_bytes)?;

        let client_id = connection_end.client_id();

        // client state query path
        let client_state_path = format!("clients/{client_id}/clientState");

        let client_state_bytes = chain
            .query_abci(IBC_QUERY_PATH, client_state_path.as_bytes(), &latest_height)
            .await?;

        let any_client_state = Any {
            type_url: TENDERMINT_CLIENT_STATE_TYPE_URL.to_owned(),
            value: client_state_bytes,
        };

        let client_state: TendermintClientState =
            CosmosChain::default_encoding().convert(&any_client_state)?;

        let counterparty_chain_id = client_state.chain_id();
        let counterparty_chain = builder.build_chain(&counterparty_chain_id.clone()).await?;

        // Retrieve source Chain summary
        let commitment_sequences =
            <CosmosChain as CanQueryPacketCommitments<CosmosChain>>::query_packet_commitments(
                &chain,
                &channel_id,
                &port_id,
            )
            .await?;

        let unreceived_sequences = <CosmosChain as CanQueryUnreceivedPacketSequences<
            CosmosChain,
        >>::query_unreceived_packet_sequences(
            &counterparty_chain,
            counterparty_channel_id,
            counterparty_port_id,
            &commitment_sequences,
        )
        .await?;

        let acks_and_height_on_counterparty = <CosmosChain as CanQueryPacketAcknowledgements<
            CosmosChain,
        >>::query_packet_acknowlegements(
            &counterparty_chain,
            counterparty_channel_id,
            counterparty_port_id,
            &commitment_sequences,
        )
        .await?;

        let unreceived_acknowledgement_sequences = if let Some((acks_on_counterparty, _)) =
            acks_and_height_on_counterparty
        {
            <CosmosChain as CanQueryUnreceivedAcksSequences<CosmosChain>>::query_unreceived_acknowledgments_sequences(
                        &chain,
                        &channel_id,
                        &port_id,
                        &acks_on_counterparty,
                    )
                    .await?
        } else {
            Vec::new()
        };

        let src_summary = PendingPackets {
            unreceived_packets: unreceived_sequences,
            unreceived_acks: unreceived_acknowledgement_sequences,
        };

        // Retrieve destination chain summary
        let commitment_sequences =
            <CosmosChain as CanQueryPacketCommitments<CosmosChain>>::query_packet_commitments(
                &counterparty_chain,
                counterparty_channel_id,
                counterparty_port_id,
            )
            .await?;

        let unreceived_sequences = <CosmosChain as CanQueryUnreceivedPacketSequences<
            CosmosChain,
        >>::query_unreceived_packet_sequences(
            &chain, &channel_id, &port_id, &commitment_sequences
        )
        .await?;

        let acks_and_height_on_counterparty = <CosmosChain as CanQueryPacketAcknowledgements<
            CosmosChain,
        >>::query_packet_acknowlegements(
            &chain,
            &channel_id,
            &port_id,
            &commitment_sequences,
        )
        .await?;

        let unreceived_acknowledgement_sequences = if let Some((acks_on_counterparty, _)) =
            acks_and_height_on_counterparty
        {
            <CosmosChain as CanQueryUnreceivedAcksSequences<CosmosChain>>::query_unreceived_acknowledgments_sequences(
                    &counterparty_chain,
                        counterparty_channel_id,
                        counterparty_port_id,
                        &acks_on_counterparty,
                    )
                    .await?
        } else {
            Vec::new()
        };

        let dst_summary = PendingPackets {
            unreceived_packets: unreceived_sequences,
            unreceived_acks: unreceived_acknowledgement_sequences,
        };

        Ok(Summary {
            src_chain: chain_id,
            dst_chain: counterparty_chain_id.clone(),
            src: src_summary,
            dst: dst_summary,
        })
    }
}

#[cgp_provider(CommandRunnerComponent)]
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
