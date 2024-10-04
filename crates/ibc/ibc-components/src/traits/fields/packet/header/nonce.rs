use core::marker::PhantomData;

use cgp::core::component::WithProvider;
use cgp::core::field::FieldGetter;
use cgp::prelude::*;

use crate::traits::types::packet::header::HasPacketHeaderType;
use crate::traits::types::packet::nonce::HasPacketNonceType;

#[derive_component(PacketNonceGetterComponent, PacketNonceGetter<Chain>)]
pub trait HasPacketNonce<Counterparty>:
    HasPacketHeaderType<Counterparty> + HasPacketNonceType<Counterparty>
{
    fn packet_nonce(packet_header: &Self::PacketHeader) -> &Self::PacketNonce;
}

impl<Chain, Counterparty, Provider> PacketNonceGetter<Chain, Counterparty>
    for WithProvider<Provider>
where
    Chain: HasPacketHeaderType<Counterparty> + HasPacketNonceType<Counterparty>,
    Provider: FieldGetter<Chain::PacketHeader, symbol!("nonce"), Field = Chain::PacketNonce>,
{
    fn packet_nonce(packet_header: &Chain::PacketHeader) -> &Chain::PacketNonce {
        Provider::get_field(packet_header, PhantomData)
    }
}
