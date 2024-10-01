use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::client_id::HasClientIdType;

use crate::traits::types::packet::nonce::HasPacketNonceType;

#[derive_component(PacketNonceAllocatorComponent, PacketNonceAllocator<Chain>)]
#[async_trait]
pub trait CanAllocatePacketNonce<Counterparty>:
    HasPacketNonceType<Counterparty> + HasClientIdType<Counterparty> + HasErrorType
where
    Counterparty: HasClientIdType<Self>,
{
    async fn allocate_packet_nonce(
        &self,
        src_client_id: &Self::ClientId,
        dst_client_id: &Counterparty::ClientId,
    ) -> Result<Self::PacketNonce, Self::Error>;
}
