use cgp::prelude::*;
use hermes_chain_type_components::traits::types::address::HasAddressType;
use hermes_chain_type_components::traits::types::amount::HasAmountType;
use hermes_ibc_components::traits::types::payload::data::ProvidePayloadDataType;

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
