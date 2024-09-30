use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::client_id::HasClientIdType;

use crate::traits::types::packet::header::HasPacketHeaderType;

#[derive_component(PacketClientGetterComponent, PacketClientGetter<Chain>)]
pub trait HasPacketClients<Counterparty>:
    HasPacketHeaderType<Counterparty> + HasClientIdType<Counterparty>
where
    Counterparty: HasClientIdType<Self>,
{
    fn packet_source_client_id(packet: &Self::PacketHeader) -> &Self::ClientId;

    fn packet_destination_client_id(packet: &Self::PacketHeader) -> &Counterparty::ClientId;
}
