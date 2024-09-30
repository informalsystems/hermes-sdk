use cgp::prelude::*;

use crate::traits::types::packet::header::HasPacketHeaderType;
use crate::traits::types::packet::timeout::HasPacketTimeoutType;

#[derive_component(PacketTimeoutGetterComponent, PacketTimeoutGetter<Chain>)]
pub trait HasPacketTimeout<Counterparty>: HasPacketHeaderType<Counterparty>
where
    Counterparty: HasPacketTimeoutType<Self>,
{
    fn packet_timeout(packet_header: &Self::PacketHeader) -> &Counterparty::PacketTimeout;
}
