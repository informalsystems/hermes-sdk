use cgp::prelude::*;

use crate::traits::types::app_id::HasAppIdType;
use crate::traits::types::payload::header::HasPayloadHeaderType;

#[derive_component(PacketApplicationGetterComponent, PacketApplicationGetter<Chain>)]
pub trait HasPacketApplications<Counterparty>:
    HasPayloadHeaderType<Counterparty> + HasAppIdType<Counterparty>
where
    Counterparty: HasAppIdType<Self>,
{
    fn packet_src_app_id(payload: &Self::PayloadHeader) -> &Self::AppId;

    fn packet_dst_app_id(payload: &Self::PayloadHeader) -> &Counterparty::AppId;
}
