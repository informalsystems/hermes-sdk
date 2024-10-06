use cgp::prelude::*;

use crate::traits::types::app_id::HasAppIdType;
use crate::traits::types::message_header::ProvideIbcMessageHeaderType;

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

impl<Chain, Counterparty> ProvideIbcMessageHeaderType<Chain, Counterparty> for UseIbcMessageHeader
where
    Chain: HasAppIdType<Counterparty>,
    Counterparty: HasAppIdType<Chain>,
{
    type IbcMessageHeader = IbcMessageHeader<Chain, Counterparty>;
}
