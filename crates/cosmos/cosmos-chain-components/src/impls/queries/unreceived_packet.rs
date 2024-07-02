use cgp_core::error::{CanRaiseError, HasErrorType};
use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::queries::unreceived_packet_sequences::UnreceivedPacketSequencesQuerier;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::packet::HasIbcPacketTypes;
use ibc_proto::ibc::core::channel::v1::query_client::QueryClient as ChannelQueryClient;
use ibc_relayer::chain::requests::QueryUnreceivedPacketsRequest;
use ibc_relayer_types::core::ics04_channel::packet::Sequence;
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, PortId};
use tonic::Request;

use crate::traits::grpc_address::HasGrpcAddress;

pub struct QueryUnreceivedCosmosPacketSequences;

#[async_trait]
impl<Chain, Counterparty> UnreceivedPacketSequencesQuerier<Chain, Counterparty>
    for QueryUnreceivedCosmosPacketSequences
where
    Chain: HasIbcChainTypes<Counterparty, ChannelId = ChannelId, PortId = PortId>
        + HasIbcPacketTypes<Counterparty>
        + HasErrorType
        + HasGrpcAddress
        + CanRaiseError<eyre::Report>,
    Counterparty: HasIbcChainTypes<Chain, Sequence = Sequence>,
{
    async fn query_unreceived_packet_sequences(
        chain: &Chain,
        channel_id: &Chain::ChannelId,
        port_id: &Chain::PortId,
        sequences: &[Counterparty::Sequence],
    ) -> Result<Vec<Counterparty::Sequence>, Chain::Error> {
        let mut client = ChannelQueryClient::connect(chain.grpc_address().clone())
            .await
            .map_err(|e| Chain::raise_error(e.into()))?;

        let raw_request = QueryUnreceivedPacketsRequest {
            port_id: port_id.clone(),
            channel_id: channel_id.clone(),
            packet_commitment_sequences: sequences.to_vec(),
        };

        let request = Request::new(raw_request.into());

        let response = client
            .unreceived_packets(request)
            .await
            .map_err(|e| Chain::raise_error(e.into()))?
            .into_inner();

        let response_sequences = response.sequences.into_iter().map(|s| s.into()).collect();
        Ok(response_sequences)
    }
}
