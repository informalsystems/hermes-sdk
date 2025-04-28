use hermes_chain_type_components::traits::HasChannelIdType;
use hermes_prelude::*;

use crate::traits::types::packet::nonce::HasPacketNonceType;
use crate::traits::types::packet::packet::HasPacketType;

#[cgp_component {
  provider: SendPacketCommitmentQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQuerySendPacketCommitment<Counterparty>:
    HasPacketType<Counterparty> + HasChannelIdType<Counterparty> + HasAsyncErrorType
where
    Counterparty: HasChannelIdType<Self> + HasPacketNonceType<Self>,
{
    async fn query_send_packet_commitment(
        &self,
        src_channel_id: &Counterparty::ChannelId,
        dst_channel_id: &Self::ChannelId,
        nonce: &Counterparty::PacketNonce,
    ) -> Result<Option<Self::Packet>, Self::Error>;
}
