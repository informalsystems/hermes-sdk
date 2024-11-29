use ibc::core::channel::types::proto::v1::query_client::QueryClient;
use ibc::core::channel::types::proto::v1::{
    QueryPacketAcknowledgementsRequest, QueryPacketCommitmentsRequest, QueryUnreceivedAcksRequest,
};
use oneline_eyre::eyre::eyre;

use hermes_cli_components::traits::build::CanLoadBuilder;
use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::{json, Output};
use hermes_cosmos_chain_components::traits::abci_query::CanQueryAbci;
use hermes_cosmos_chain_components::traits::grpc_address::HasGrpcAddress;
use hermes_cosmos_relayer::contexts::build::CosmosBuilder;
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainHeight;

use ibc::primitives::proto::Protobuf;
use ibc_relayer_types::core::ics04_channel::channel::ChannelEnd;
use ibc_relayer_types::core::ics04_channel::channel::State;
use ibc_relayer_types::core::ics04_channel::packet::Sequence;
use ibc_relayer_types::core::ics24_host::{
    identifier::{ChainId, ChannelId, PortId},
    IBC_QUERY_PATH,
};
use ibc_relayer_types::Height;

use crate::commands::query::packet::util::PacketSequences;
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
    async fn execute(&self, builder: &CosmosBuilder) -> Result<Option<(Vec<Sequence>, Height)>> {
        let port_id = self.port_id.clone();
        let channel_id = self.channel_id.clone();
        let chain = builder.build_chain(&self.chain_id).await?;

        let latest_height = chain.query_chain_height().await?;

        // channel end query path
        let channel_end_path = format!("channelEnds/ports/{port_id}/channels/{channel_id}");

        let channel_end_bytes = chain
            .query_abci(IBC_QUERY_PATH, channel_end_path.as_bytes(), &latest_height)
            .await?;

        let channel_end = ChannelEnd::decode_vec(&channel_end_bytes).unwrap();

        // check if channel end is initialized, otherwize return error.
        if channel_end.state_matches(&State::Uninitialized) {
            return Err(eyre!("channel with id `{channel_id}` is uninitialized").into());
        }

        let mut client = QueryClient::connect(chain.grpc_address().clone())
            .await
            .unwrap();

        let req = tonic::Request::new(QueryPacketCommitmentsRequest {
            port_id: port_id.to_string(),
            channel_id: channel_id.to_string(),
            pagination: None,
        });

        let response = client.packet_commitments(req).await.unwrap().into_inner();

        let commitment_sequences = response
            .commitments
            .iter()
            .map(|commitment| commitment.sequence)
            .collect();

        let req = tonic::Request::new(QueryPacketAcknowledgementsRequest {
            port_id: port_id.to_string(),
            channel_id: channel_id.to_string(),
            pagination: None,
            packet_commitment_sequences: commitment_sequences,
        });

        let response = client
            .packet_acknowledgements(req)
            .await
            .unwrap()
            .into_inner();

        let response_height = Height::try_from(response.height.unwrap()).unwrap();

        let ack_sequences = response
            .acknowledgements
            .iter()
            .map(|ack| ack.sequence)
            .collect();

        let request = tonic::Request::new(QueryUnreceivedAcksRequest {
            port_id: port_id.to_string(),
            channel_id: channel_id.to_string(),
            packet_ack_sequences: ack_sequences,
        });

        let response = client.unreceived_acks(request).await.unwrap().into_inner();

        let unreceived_ack_sequences = response
            .sequences
            .iter()
            .map(|sequence| Sequence::from(*sequence))
            .collect();

        Ok(Some((unreceived_ack_sequences, response_height)))
    }
}

impl CommandRunner<HermesApp> for QueryPendingAcks {
    async fn run(&self, app: &HermesApp) -> Result<Output> {
        let builder = app.load_builder().await?;

        match self.execute(&builder).await {
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
