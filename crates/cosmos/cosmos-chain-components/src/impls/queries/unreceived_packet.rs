use cgp::core::error::{CanRaiseAsyncError, HasAsyncErrorType};
use hermes_relayer_components::chain::traits::queries::unreceived_packet_sequences::UnreceivedPacketSequencesQuerier;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use http::uri::InvalidUri;
use http::Uri;
use ibc::core::host::types::identifiers::{ChannelId, PortId, Sequence};
use ibc_proto::ibc::core::channel::v1::query_client::QueryClient as ChannelQueryClient;
use ibc_proto::ibc::core::channel::v1::QueryUnreceivedPacketsRequest;
use tonic::transport::Error as TransportError;
use tonic::{Request, Status};

use crate::traits::grpc_address::HasGrpcAddress;

pub struct QueryUnreceivedCosmosPacketSequences;

impl<Chain, Counterparty> UnreceivedPacketSequencesQuerier<Chain, Counterparty>
    for QueryUnreceivedCosmosPacketSequences
where
    Chain: HasIbcChainTypes<Counterparty, ChannelId = ChannelId, PortId = PortId>
        + HasAsyncErrorType
        + HasGrpcAddress
        + CanRaiseAsyncError<InvalidUri>
        + CanRaiseAsyncError<TransportError>
        + CanRaiseAsyncError<Status>
        + CanRaiseAsyncError<eyre::Report>,
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
            port_id: port_id.to_string(),
            channel_id: channel_id.to_string(),
            packet_commitment_sequences: sequences
                .iter()
                .map(|sequence| sequence.value())
                .collect(),
        };

        let request = Request::new(raw_request);

        let response = client
            .unreceived_packets(request)
            .await
            .map_err(Chain::raise_error)?
            .into_inner();

        let response_sequences = response.sequences.into_iter().map(|s| s.into()).collect();
        Ok(response_sequences)
    }
}
