use cgp::prelude::*;
use hermes_chain_type_components::traits::HasChannelIdType;

use crate::traits::types::packet::nonce::HasPacketNonceType;

#[cgp_component {
  provider: HasPacketReceivedQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQueryHasPacketReceived<Counterparty>:
    HasChannelIdType<Counterparty> + HasAsyncErrorType
where
    Counterparty: HasChannelIdType<Self> + HasPacketNonceType<Self>,
{
    async fn query_has_packet_received(
        &self,
        src_channel_id: &Counterparty::ChannelId,
        dst_channel_id: &Self::ChannelId,
        nonce: &Counterparty::PacketNonce,
    ) -> Result<bool, Self::Error>;
}
