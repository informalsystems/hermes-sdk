use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;

use crate::traits::types::packet::nonce::HasPacketNonceType;

#[derive_component(PacketNonceAllocatorComponent, PacketNonceAllocator<Chain>)]
#[async_trait]
pub trait CanAllocatePacketNonce<Counterparty>:
    HasPacketNonceType<Counterparty> + HasChannelIdType<Counterparty> + HasErrorType
where
    Counterparty: HasChannelIdType<Self>,
{
    async fn allocate_packet_nonce(
        &self,
        src_channel_id: &Self::ChannelId,
        dst_channel_id: &Counterparty::ChannelId,
    ) -> Result<Self::PacketNonce, Self::Error>;
}
