use cgp::prelude::*;

use crate::traits::types::app_id::HasAppIdType;
use crate::traits::types::message_header::{
    IbcMessageHeaderTypeComponent, ProvideIbcMessageHeaderType,
};

#[derive(HasField)]
pub struct IbcMessageHeader<Chain, Counterparty>
where
    Chain: HasAppIdType<Counterparty>,
    Counterparty: HasAppIdType<Chain>,
{
    pub src_app_id: Chain::AppId,
    pub dst_app_id: Counterparty::AppId,
}

pub struct UseIbcMessageHeader;

#[cgp_provider(IbcMessageHeaderTypeComponent)]
impl<Chain, Counterparty> ProvideIbcMessageHeaderType<Chain, Counterparty> for UseIbcMessageHeader
where
    Chain: HasAppIdType<Counterparty>,
    Counterparty: HasAppIdType<Chain>,
{
    type IbcMessageHeader = IbcMessageHeader<Chain, Counterparty>;
}

impl<Chain, Counterparty> Clone for IbcMessageHeader<Chain, Counterparty>
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
