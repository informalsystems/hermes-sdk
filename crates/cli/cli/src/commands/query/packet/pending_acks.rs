use cgp::prelude::*;
use hermes_chain_components::traits::queries::packet_acknowledgements::CanQueryPacketAcknowledgements;
use hermes_chain_components::traits::queries::packet_commitments::CanQueryPacketCommitments;
use hermes_chain_components::traits::queries::unreceived_acks_sequences::CanQueryUnreceivedAcksSequences;
use hermes_cli_components::traits::build::CanLoadBuilder;
use hermes_cli_components::traits::command::CommandRunnerComponent;
use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;
use hermes_cosmos_chain_components::traits::abci_query::CanQueryAbci;
use hermes_cosmos_chain_components::types::tendermint::TendermintClientState;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_encoding_components::traits::convert::CanConvert;
use hermes_encoding_components::traits::has_encoding::HasDefaultEncoding;
use hermes_protobuf_encoding_components::types::any::Any;
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainHeight;
use ibc::clients::tendermint::types::TENDERMINT_CLIENT_STATE_TYPE_URL;
use ibc::core::channel::types::channel::{ChannelEnd, State};
use ibc::core::connection::types::ConnectionEnd;
use ibc::core::host::types::identifiers::{ChainId, ChannelId, PortId, Sequence};
use ibc::cosmos_host::IBC_QUERY_PATH;
use ibc::primitives::proto::Protobuf;
use oneline_eyre::eyre::eyre;

use crate::contexts::app::HermesApp;
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
    async fn execute(&self, builder: &CosmosBuilder) -> Result<Option<Vec<Sequence>>> {
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

        // check if channel end is initialized, otherwize return error.
        if channel_end
            .verify_state_matches(&State::Uninitialized)
            .is_ok()
        {
            return Err(eyre!("channel with id `{channel_id}` is uninitialized").into());
        }

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

        let commitment_sequences =
            <CosmosChain as CanQueryPacketCommitments<CosmosChain>>::query_packet_commitments(
                &chain,
                &channel_id,
                &port_id,
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

        let unreceived_acknowledgement_sequences_and_height = if let Some(acks_on_counterparty) =
            acks_and_height_on_counterparty
        {
            Some(<CosmosChain as CanQueryUnreceivedAcksSequences<CosmosChain>>::query_unreceived_acknowledgments_sequences(
                        &chain,
                        &channel_id,
                        &port_id,
                        &acks_on_counterparty,
                    )
                    .await?)
        } else {
            None
        };

        Ok(unreceived_acknowledgement_sequences_and_height)
    }
}

#[cgp_provider(CommandRunnerComponent)]
impl CommandRunner<HermesApp> for QueryPendingAcks {
    async fn run(&self, app: &HermesApp) -> Result<Output> {
        let builder = app.load_builder().await?;

        match self.execute(&builder).await {
            Err(e) => Ok(Output::error(e)),
            Ok(None) => Ok(Output::success_msg("No unreceived acknowledgements")),
            Ok(Some(sequences)) => Ok(Output::success(sequences)),
        }
    }
}
