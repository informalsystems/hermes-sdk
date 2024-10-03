use cgp::prelude::*;

use crate::traits::types::app_id::HasAppIdType;
use crate::traits::types::payload::header::HasPayloadHeaderType;

#[derive_component(PayloadAppIdGetterComponent, PayloadAppIdGetter<Chain>)]
pub trait HasPayloadAppIds<Counterparty>:
    HasPayloadHeaderType<Counterparty> + HasAppIdType<Counterparty>
where
    Counterparty: HasAppIdType<Self>,
{
    fn payload_src_app_id(payload_header: &Self::PayloadHeader) -> &Self::AppId;

    fn payload_dst_app_id(payload_header: &Self::PayloadHeader) -> &Counterparty::AppId;
}
