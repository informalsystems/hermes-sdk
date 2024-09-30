use cgp::prelude::*;

use crate::traits::types::app_id::HasAppIdType;
use crate::traits::types::packet::payload::HasPacketPayloadHeaderType;

#[derive_component(PacketApplicationGetterComponent, PacketApplicationGetter<Chain>)]
pub trait HasPacketApplications<Counterparty>:
    HasPacketPayloadHeaderType<Counterparty> + HasAppIdType<Counterparty>
where
    Counterparty: HasAppIdType<Self>,
{
    fn source_app_id(payload: &Self::PacketPayloadHeader) -> &Self::AppId;

    fn destination_app_id(payload: &Self::PacketPayloadHeader) -> &Counterparty::AppId;
}
