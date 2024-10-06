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
