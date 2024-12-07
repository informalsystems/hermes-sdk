use core::marker::PhantomData;

use cgp::core::component::WithProvider;
use cgp::core::field::FieldGetter;
use cgp::prelude::*;

use crate::traits::types::packet::nonce::HasPacketNonceType;
use crate::traits::types::packet::packet::HasPacketType;

/**
   Getter for the nonce from the packet.

   The nonce is expected to be placed directly in the packet, separate from
   the packet header. This is because the packet header type is also used
   in places where the nonce has not yet become available.

   By placing the nonce within the packet itself, we can avoid defining
   a separate header type that mirrors all fields in the packet header
   except the nonce.
*/
#[derive_component(PacketNonceGetterComponent, PacketNonceGetter<Chain>)]
pub trait HasPacketNonce<Counterparty>:
    HasPacketType<Counterparty> + HasPacketNonceType<Counterparty>
{
    fn packet_nonce(packet_header: &Self::Packet) -> &Self::PacketNonce;
}

impl<Chain, Counterparty, Provider> PacketNonceGetter<Chain, Counterparty>
    for WithProvider<Provider>
where
    Chain: HasPacketType<Counterparty> + HasPacketNonceType<Counterparty>,
    Provider: FieldGetter<Chain::Packet, symbol!("nonce"), Value = Chain::PacketNonce>,
{
    fn packet_nonce(packet_header: &Chain::Packet) -> &Chain::PacketNonce {
        Provider::get_field(packet_header, PhantomData)
    }
}
