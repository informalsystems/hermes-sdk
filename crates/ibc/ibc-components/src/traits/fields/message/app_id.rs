use crate::traits::types::app_id::HasAppIdType;
use crate::traits::types::message::HasIbcMessageType;

pub trait HasIbcMessageAppIds<Counterparty, App>:
    HasIbcMessageType<Counterparty, App> + HasAppIdType<Counterparty>
where
    Counterparty: HasAppIdType<Self>,
{
    fn ibc_message_src_app_id(message: &Self::IbcMessage) -> &Self::AppId;

    fn ibc_message_dst_app_id(message: &Self::IbcMessage) -> &Counterparty::AppId;
}
