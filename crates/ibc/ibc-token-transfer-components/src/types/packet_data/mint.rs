use cgp::prelude::*;
use hermes_chain_type_components::traits::types::address::HasAddressType;
use hermes_chain_type_components::traits::types::amount::HasAmountType;
use hermes_ibc_components::traits::types::payload::data::{
    PayloadDataTypeComponent, ProvidePayloadDataType,
};

#[derive(HasField)]
pub struct IbcTransferMintPayloadData<Chain, Counterparty>
where
    Chain: HasAmountType,
    Counterparty: HasAddressType,
{
    pub receiver: Counterparty::Address,
    pub amount: Chain::Amount,
}

pub struct UseIbcTransferMintPayloadData;

#[cgp_provider(PayloadDataTypeComponent)]
impl<Chain, Counterparty, App> ProvidePayloadDataType<Chain, Counterparty, App>
    for UseIbcTransferMintPayloadData
where
    Chain: HasAmountType,
    Counterparty: HasAddressType,
{
    type PayloadData = IbcTransferMintPayloadData<Chain, Counterparty>;
}

impl<Chain, Counterparty> Clone for IbcTransferMintPayloadData<Chain, Counterparty>
where
    Chain: HasAmountType<Amount: Clone>,
    Counterparty: HasAddressType<Address: Clone>,
{
    fn clone(&self) -> Self {
        Self {
            receiver: self.receiver.clone(),
            amount: self.amount.clone(),
        }
    }
}

impl<Chain, Counterparty> PartialEq for IbcTransferMintPayloadData<Chain, Counterparty>
where
    Chain: HasAmountType<Amount: Eq>,
    Counterparty: HasAddressType<Address: Eq>,
{
    fn eq(&self, other: &Self) -> bool {
        self.receiver == other.receiver && self.amount == other.amount
    }
}

impl<Chain, Counterparty> Eq for IbcTransferMintPayloadData<Chain, Counterparty>
where
    Chain: HasAmountType<Amount: Eq>,
    Counterparty: HasAddressType<Address: Eq>,
{
}
