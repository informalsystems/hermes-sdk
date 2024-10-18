use cgp::prelude::*;
use hermes_chain_type_components::traits::types::address::HasAddressType;
use hermes_chain_type_components::traits::types::amount::HasAmountType;
use hermes_ibc_components::traits::types::message::ProvideIbcMessageType;

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

impl<Chain, Counterparty, App> ProvideIbcMessageType<Chain, Counterparty, App>
    for UseIbcTransferMessage
where
    Chain: HasAmountType,
    Counterparty: HasAddressType,
{
    type IbcMessage = IbcTransferMessage<Chain, Counterparty>;
}
