use cgp::prelude::*;

use crate::traits::types::app_id::HasAppIdType;
use crate::traits::types::message_header::HasIbcMessageHeaderType;

#[derive_component(IbcMessageAppIdGetterComponent, IbcMessageAppIdGetter<Chain>)]
pub trait HasIbcMessageAppIds<Counterparty>:
    HasIbcMessageHeaderType<Counterparty> + HasAppIdType<Counterparty>
where
    Counterparty: HasAppIdType<Self>,
{
    fn ibc_message_src_app_id(message_header: &Self::IbcMessageHeader) -> &Self::AppId;

    fn ibc_message_dst_app_id(message_header: &Self::IbcMessageHeader) -> &Counterparty::AppId;
}
