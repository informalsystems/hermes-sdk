use cgp::prelude::CanRaiseError;
use hermes_relayer_components::chain::traits::queries::packet_is_received::ReceivedPacketQuerier;
use hermes_relayer_components::chain::traits::queries::unreceived_packet_sequences::CanQueryUnreceivedPacketSequences;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc::core::host::types::identifiers::{ChannelId, PortId, Sequence};
use tonic::transport::Error as TransportError;
use tonic::Status;

use crate::traits::grpc_address::HasGrpcAddress;

pub struct QueryCosmosReceivedPacket;

impl<Chain, Counterparty> ReceivedPacketQuerier<Chain, Counterparty> for QueryCosmosReceivedPacket
where
    Chain: HasIbcChainTypes<Counterparty, ChannelId = ChannelId, PortId = PortId>
        + HasGrpcAddress
        + CanQueryUnreceivedPacketSequences<Counterparty>
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
        let sequence = *sequence;

        let unreceived_sequences = chain
            .query_unreceived_packet_sequences(channel_id, port_id, &[sequence])
            .await?;

        Ok(unreceived_sequences.is_empty())
    }
}
