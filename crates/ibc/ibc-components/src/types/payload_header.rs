use cgp::prelude::*;

use crate::traits::types::app_id::HasAppIdType;
use crate::traits::types::payload::header::ProvidePayloadHeaderType;

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
