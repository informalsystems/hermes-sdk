use cgp::prelude::*;
use hermes_chain_type_components::traits::types::address::HasAddressType;
use hermes_chain_type_components::traits::types::amount::HasAmountType;

pub enum IbcTransferAmount<Chain, Counterparty>
where
    Chain: HasAmountType,
    Counterparty: HasAmountType,
{
    Mint(Chain::Amount),
    Unescrow(Counterparty::Amount),
}

#[derive(HasField)]
pub struct IbcTransferPacketData<Chain, Counterparty>
where
    Chain: HasAmountType,
    Counterparty: HasAmountType + HasAddressType,
{
    pub receiver: Counterparty::Address,
    pub amount: IbcTransferAmount<Chain, Counterparty>,
}
