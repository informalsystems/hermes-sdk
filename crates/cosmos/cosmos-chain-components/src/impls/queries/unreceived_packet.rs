use cgp::core::error::{CanRaiseError, HasErrorType};
use hermes_relayer_components::chain::traits::queries::unreceived_packet_sequences::UnreceivedPacketSequencesQuerier;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use http::uri::InvalidUri;
use http::Uri;
use tonic::transport::Error as TransportError;
use tonic::Status;

use ibc_proto::ibc::core::channel::v1::query_client::QueryClient as ChannelQueryClient;
use ibc_relayer::chain::requests::QueryUnreceivedPacketsRequest;
use ibc_relayer_types::core::ics04_channel::packet::Sequence;
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, PortId};
use tonic::Request;

use crate::traits::grpc_address::HasGrpcAddress;

pub struct QueryUnreceivedCosmosPacketSequences;

impl<Chain, Counterparty> UnreceivedPacketSequencesQuerier<Chain, Counterparty>
    for QueryUnreceivedCosmosPacketSequences
where
    Chain: HasIbcChainTypes<Counterparty, ChannelId = ChannelId, PortId = PortId>
        + HasErrorType
        + HasGrpcAddress
        + CanRaiseError<InvalidUri>
        + CanRaiseError<TransportError>
        + CanRaiseError<Status>
        + CanRaiseError<eyre::Report>,
    Counterparty: HasIbcChainTypes<Chain, Sequence = Sequence>,
{
    async fn query_unreceived_packet_sequences(
        chain: &Chain,
        channel_id: &Chain::ChannelId,
        port_id: &Chain::PortId,
        sequences: &[Counterparty::Sequence],
    ) -> Result<Vec<Counterparty::Sequence>, Chain::Error> {
        let mut client = ChannelQueryClient::connect(
            Uri::try_from(&chain.grpc_address().to_string()).map_err(Chain::raise_error)?,
        )
        .await
        .map_err(Chain::raise_error)?;

        let raw_request = QueryUnreceivedPacketsRequest {
            port_id: port_id.clone(),
            channel_id: channel_id.clone(),
            packet_commitment_sequences: sequences.to_vec(),
        };

        let request = Request::new(raw_request.into());

        let response = client
            .unreceived_packets(request)
            .await
            .map_err(Chain::raise_error)?
            .into_inner();

        let response_sequences = response.sequences.into_iter().map(|s| s.into()).collect();
        Ok(response_sequences)
    }
}
