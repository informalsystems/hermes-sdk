use hermes_chain_type_components::traits::{HasAddressType, HasAmountType};
use hermes_ibc_components::traits::types::message::{
    IbcMessageTypeComponent, ProvideIbcMessageType,
};
use hermes_prelude::*;

#[derive(HasField)]
pub struct IbcTransferMessage<Chain, Counterparty>
where
    Chain: HasAmountType,
    Counterparty: HasAddressType,
{
    pub amount: Chain::Amount,
    pub receiver: Counterparty::Address,
}

pub struct UseIbcTransferMessage;

#[cgp_provider(IbcMessageTypeComponent)]
impl<Chain, Counterparty, App> ProvideIbcMessageType<Chain, Counterparty, App>
    for UseIbcTransferMessage
where
    Chain: HasAmountType,
    Counterparty: HasAddressType,
{
    type IbcMessage = IbcTransferMessage<Chain, Counterparty>;
}
