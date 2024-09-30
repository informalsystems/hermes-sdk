use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::client_id::HasClientIdType;

use crate::traits::types::packet::header::HasPacketHeaderType;

#[derive_component(PacketClientGetterComponent, PacketClientGetter<Chain>)]
pub trait HasPacketClients<Counterparty>:
    HasPacketHeaderType<Counterparty> + HasClientIdType<Counterparty>
where
    Counterparty: HasClientIdType<Self>,
{
    fn packet_src_client_id(packet_header: &Self::PacketHeader) -> &Self::ClientId;

    fn packet_dst_client_id(packet_header: &Self::PacketHeader) -> &Counterparty::ClientId;
}
