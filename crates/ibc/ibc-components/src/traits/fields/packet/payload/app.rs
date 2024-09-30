use cgp::prelude::*;

use crate::traits::types::app_id::HasAppIdType;
use crate::traits::types::packet::entry::HasPacketEntryHeaderType;

#[derive_component(PacketApplicationGetterComponent, PacketApplicationGetter<Chain>)]
pub trait HasPacketApplications<Counterparty>:
    HasPacketEntryHeaderType<Counterparty> + HasAppIdType<Counterparty>
where
    Counterparty: HasAppIdType<Self>,
{
    fn source_app_id(payload: &Self::PacketEntryHeader) -> &Self::AppId;

    fn destination_app_id(payload: &Self::PacketEntryHeader) -> &Counterparty::AppId;
}
