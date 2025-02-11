use cgp::prelude::*;

use crate::traits::types::app_id::HasAppIdType;
use crate::traits::types::payload::header::{PayloadHeaderTypeComponent, ProvidePayloadHeaderType};
use crate::types::message_header::IbcMessageHeader;

#[derive(HasField)]
pub struct IbcPayloadHeader<Chain, Counterparty>
where
    Chain: HasAppIdType<Counterparty>,
    Counterparty: HasAppIdType<Chain>,
{
    pub src_app_id: Chain::AppId,
    pub dst_app_id: Counterparty::AppId,
}

pub struct UseIbcPayloadHeader;

#[cgp_provider(PayloadHeaderTypeComponent)]
impl<Chain, Counterparty> ProvidePayloadHeaderType<Chain, Counterparty> for UseIbcPayloadHeader
where
    Chain: HasAppIdType<Counterparty>,
    Counterparty: HasAppIdType<Chain>,
{
    type PayloadHeader = IbcPayloadHeader<Chain, Counterparty>;
}

impl<Chain, Counterparty> Clone for IbcPayloadHeader<Chain, Counterparty>
where
    Chain: HasAppIdType<Counterparty, AppId: Clone>,
    Counterparty: HasAppIdType<Chain, AppId: Clone>,
{
    fn clone(&self) -> Self {
        Self {
            src_app_id: self.src_app_id.clone(),
            dst_app_id: self.dst_app_id.clone(),
        }
    }
}

impl<Chain, Counterparty> PartialEq for IbcPayloadHeader<Chain, Counterparty>
where
    Chain: HasAppIdType<Counterparty, AppId: Eq>,
    Counterparty: HasAppIdType<Chain, AppId: Eq>,
{
    fn eq(&self, other: &Self) -> bool {
        self.src_app_id == other.src_app_id && self.dst_app_id == other.dst_app_id
    }
}

impl<Chain, Counterparty> Eq for IbcPayloadHeader<Chain, Counterparty>
where
    Chain: HasAppIdType<Counterparty, AppId: Eq>,
    Counterparty: HasAppIdType<Chain, AppId: Eq>,
{
}

impl<Chain, Counterparty> From<IbcMessageHeader<Chain, Counterparty>>
    for IbcPayloadHeader<Chain, Counterparty>
where
    Chain: HasAppIdType<Counterparty>,
    Counterparty: HasAppIdType<Chain>,
{
    fn from(header: IbcMessageHeader<Chain, Counterparty>) -> Self {
        Self {
            src_app_id: header.src_app_id,
            dst_app_id: header.dst_app_id,
        }
    }
}
