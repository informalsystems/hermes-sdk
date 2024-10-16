use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_type_components::traits::types::address::HasAddressType;
use hermes_chain_type_components::traits::types::amount::HasAmountType;
use hermes_ibc_components::traits::types::payload::data::ProvidePayloadDataType;

#[derive(HasField)]
pub struct IbcTransferUnescrowPayloadData<Chain, Counterparty>
where
    Counterparty: HasAmountType + HasAddressType,
{
    pub receiver: Counterparty::Address,
    pub amount: Counterparty::Amount,
    pub phantom: PhantomData<Chain>,
}

pub struct UseIbcTransferUnescrowPayloadData;

impl<Chain, Counterparty, App> ProvidePayloadDataType<Chain, Counterparty, App>
    for UseIbcTransferUnescrowPayloadData
where
    Chain: Async,
    Counterparty: HasAmountType + HasAddressType,
{
    type PayloadData = IbcTransferUnescrowPayloadData<Chain, Counterparty>;
}

impl<Chain, Counterparty> Clone for IbcTransferUnescrowPayloadData<Chain, Counterparty>
where
    Counterparty: HasAmountType<Amount: Clone> + HasAddressType<Address: Clone>,
{
    fn clone(&self) -> Self {
        Self {
            receiver: self.receiver.clone(),
            amount: self.amount.clone(),
            phantom: PhantomData,
        }
    }
}
