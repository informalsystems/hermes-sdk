use cgp::prelude::*;

use crate::traits::types::app_id::HasAppIdType;

#[derive(HasField)]
pub struct IbcPayloadHeader<Chain, Counterparty>
where
    Chain: HasAppIdType<Counterparty>,
    Counterparty: HasAppIdType<Chain>,
{
    pub src_app_id: Chain::AppId,
    pub dst_app_id: Counterparty::AppId,
}
