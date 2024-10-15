use cgp::prelude::*;
use hermes_chain_type_components::traits::types::address::HasAddressType;
use hermes_chain_type_components::traits::types::amount::HasAmountType;
use hermes_ibc_components::traits::types::payload::data::ProvidePayloadDataType;

#[derive(HasField)]
pub struct IbcTransferPayloadData<Chain, Counterparty>
where
    Chain: HasAmountType,
    Counterparty: HasAmountType + HasAddressType,
{
    pub receiver: Counterparty::Address,
    pub amount: IbcTransferAmount<Chain, Counterparty>,
}

pub enum IbcTransferAmount<Chain, Counterparty>
where
    Chain: HasAmountType,
    Counterparty: HasAmountType,
{
    Mint(Chain::Amount),
    Unescrow(Counterparty::Amount),
}

pub struct UseIbcTransferPayloadData;

impl<Chain, Counterparty, App> ProvidePayloadDataType<Chain, Counterparty, App>
    for UseIbcTransferPayloadData
where
    Chain: HasAmountType,
    Counterparty: HasAmountType + HasAddressType,
{
    type PayloadData = IbcTransferPayloadData<Chain, Counterparty>;
}

impl<Chain, Counterparty> Clone for IbcTransferPayloadData<Chain, Counterparty>
where
    Chain: HasAmountType<Amount: Clone>,
    Counterparty: HasAmountType<Amount: Clone> + HasAddressType<Address: Clone>,
{
    fn clone(&self) -> Self {
        Self {
            receiver: self.receiver.clone(),
            amount: self.amount.clone(),
        }
    }
}

impl<Chain, Counterparty> Clone for IbcTransferAmount<Chain, Counterparty>
where
    Chain: HasAmountType<Amount: Clone>,
    Counterparty: HasAmountType<Amount: Clone>,
{
    fn clone(&self) -> Self {
        match self {
            Self::Mint(amount) => Self::Mint(amount.clone()),
            Self::Unescrow(amount) => Self::Unescrow(amount.clone()),
        }
    }
}
