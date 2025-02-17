use cgp::prelude::*;
use hermes_relayer_components::chain::traits::queries::packet_is_received::{
    PacketIsReceivedQuerier, PacketIsReceivedQuerierComponent,
};
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use http::uri::InvalidUri;
use http::Uri;
use ibc::core::host::types::identifiers::{ChannelId, PortId, Sequence};
use ibc_proto::ibc::core::channel::v1::query_client::QueryClient as ChannelQueryClient;
use ibc_proto::ibc::core::channel::v1::QueryUnreceivedPacketsRequest;
use tonic::transport::Error as TransportError;
use tonic::{Request, Status};

use crate::traits::grpc_address::HasGrpcAddress;

pub struct QueryCosmosPacketIsReceived;

#[cgp_provider(PacketIsReceivedQuerierComponent)]
impl<Chain, Counterparty> PacketIsReceivedQuerier<Chain, Counterparty>
    for QueryCosmosPacketIsReceived
where
    Chain: HasIbcChainTypes<Counterparty, ChannelId = ChannelId, PortId = PortId>
        + HasGrpcAddress
        + CanRaiseAsyncError<InvalidUri>
        + CanRaiseAsyncError<TransportError>
        + CanRaiseAsyncError<Status>,
    Counterparty: HasIbcChainTypes<Chain, Sequence = Sequence>,
{
    async fn query_packet_is_received(
        chain: &Chain,
        port_id: &Chain::PortId,
        channel_id: &Chain::ChannelId,
        sequence: &Counterparty::Sequence,
    ) -> Result<bool, Chain::Error> {
        let sequence = *sequence;

        let mut client = ChannelQueryClient::connect(
            Uri::try_from(&chain.grpc_address().to_string()).map_err(Chain::raise_error)?,
        )
        .await
        .map_err(Chain::raise_error)?;

        let raw_request = QueryUnreceivedPacketsRequest {
            port_id: port_id.to_string(),
            channel_id: channel_id.to_string(),
            packet_commitment_sequences: vec![sequence.value()],
        };

        let request = Request::new(raw_request);

        let response = client
            .unreceived_packets(request)
            .await
            .map_err(Chain::raise_error)?
            .into_inner();

        Ok(response.sequences.is_empty())
    }
}
