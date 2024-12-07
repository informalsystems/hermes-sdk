use core::marker::PhantomData;

use cgp::core::component::WithProvider;
use cgp::core::field::FieldGetter;
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

impl<Chain, Counterparty, Provider> PacketTimeoutGetter<Chain, Counterparty>
    for WithProvider<Provider>
where
    Chain: HasPacketHeaderType<Counterparty>,
    Provider:
        FieldGetter<Chain::PacketHeader, symbol!("timeout"), Value = Counterparty::PacketTimeout>,
    Counterparty: HasPacketTimeoutType<Chain>,
{
    fn packet_timeout(packet_header: &Chain::PacketHeader) -> &Counterparty::PacketTimeout {
        Provider::get_field(packet_header, PhantomData)
    }
}
