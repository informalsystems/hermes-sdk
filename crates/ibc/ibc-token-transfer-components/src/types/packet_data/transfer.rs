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

pub struct ProvideIbcTransferPayloadData;

impl<Chain, Counterparty, App> ProvidePayloadDataType<Chain, Counterparty, App>
    for ProvideIbcTransferPayloadData
where
    Chain: HasAmountType,
    Counterparty: HasAmountType + HasAddressType,
{
    type PayloadData = IbcTransferPayloadData<Chain, Counterparty>;
}
