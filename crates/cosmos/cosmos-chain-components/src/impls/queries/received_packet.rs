use cgp::prelude::CanRaiseError;
use tonic::transport::Error as TransportError;
use tonic::Status;

use hermes_relayer_components::chain::traits::queries::packet_is_received::ReceivedPacketQuerier;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;

use ibc_relayer::chain::requests::QueryUnreceivedPacketsRequest;
use ibc_relayer_types::core::ics04_channel::packet::Sequence;
use ibc_relayer_types::core::ics24_host::identifier::{ChannelId, PortId};

use ibc_proto::ibc::core::channel::v1::query_client::QueryClient;

use crate::traits::grpc_address::HasGrpcAddress;

pub struct QueryCosmosReceivedPacket;

impl<Chain, Counterparty> ReceivedPacketQuerier<Chain, Counterparty> for QueryCosmosReceivedPacket
where
    Chain: HasIbcChainTypes<Counterparty, ChannelId = ChannelId, PortId = PortId>
        + HasGrpcAddress
        + CanRaiseError<TransportError>
        + CanRaiseError<Status>,
    Counterparty: HasIbcChainTypes<Chain, Sequence = Sequence>,
{
    async fn query_packet_is_received(
        chain: &Chain,
        port_id: &Chain::PortId,
        channel_id: &Chain::ChannelId,
        sequence: &Counterparty::Sequence,
    ) -> Result<bool, Chain::Error> {
        let port_id = port_id.clone();
        let channel_id = channel_id.clone();
        let sequence = *sequence;

        let mut client = QueryClient::connect(chain.grpc_address().clone())
            .await
            .map_err(Chain::raise_error)?;

        let raw_req = QueryUnreceivedPacketsRequest {
            port_id: port_id.clone(),
            channel_id: channel_id.clone(),
            packet_commitment_sequences: vec![sequence],
        };

        let req = tonic::Request::new(raw_req.into());

        let response = client
            .unreceived_packets(req)
            .await
            .map_err(Chain::raise_error)?
            .into_inner();

        Ok(response.sequences.is_empty())
    }
}
