use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;

use crate::traits::types::packet::ack::HasPacketAckType;
use crate::traits::types::packet::nonce::HasPacketNonceType;

#[derive_component(AckPacketCommitmentQuerierComponent, AckPacketCommitmentQuerier<Chain>)]
#[async_trait]
pub trait CanQueryAckPacketCommitment<Counterparty>:
    HasPacketAckType<Counterparty> + HasChannelIdType<Counterparty> + HasErrorType
where
    Counterparty: HasChannelIdType<Self> + HasPacketNonceType<Self>,
{
    async fn query_ack_packet_commitment(
        &self,
        src_channel_id: &Counterparty::ChannelId,
        dst_channel_id: &Self::ChannelId,
        nonce: &Counterparty::PacketNonce,
    ) -> Result<Option<Self::PacketAck>, Self::Error>;
}
