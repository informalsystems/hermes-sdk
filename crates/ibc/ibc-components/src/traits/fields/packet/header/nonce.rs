use cgp::prelude::*;

use crate::traits::types::packet::header::HasPacketHeaderType;
use crate::traits::types::packet::nonce::HasPacketNonceType;

#[derive_component(PacketNonceGetterComponent, PacketNonceGetter<Chain>)]
pub trait HasPacketNonce<Counterparty>:
    HasPacketHeaderType<Counterparty> + HasPacketNonceType<Counterparty>
{
    fn packet_nonce(packet_header: &Self::PacketHeader) -> &Self::PacketNonce;
}
