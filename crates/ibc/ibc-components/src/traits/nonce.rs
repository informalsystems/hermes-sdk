use hermes_chain_type_components::traits::HasChannelIdType;
use hermes_prelude::*;

use crate::traits::types::packet::nonce::HasPacketNonceType;

#[cgp_component {
  provider: PacketNonceAllocator,
  context: Chain,
}]
#[async_trait]
pub trait CanAllocatePacketNonce<Counterparty>:
    HasPacketNonceType<Counterparty> + HasChannelIdType<Counterparty> + HasAsyncErrorType
where
    Counterparty: HasChannelIdType<Self>,
{
    async fn allocate_packet_nonce(
        &mut self,
        src_channel_id: &Self::ChannelId,
        dst_channel_id: &Counterparty::ChannelId,
    ) -> Result<Self::PacketNonce, Self::Error>;
}
