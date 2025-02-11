use cgp::prelude::*;
use hermes_chain_type_components::traits::types::address::HasAddressType;
use hermes_chain_type_components::traits::types::amount::HasAmountType;
use hermes_ibc_components::traits::types::payload::data::{
    PayloadDataTypeComponent, ProvidePayloadDataType,
};

use crate::types::packet_data::mint::IbcTransferMintPayloadData;
use crate::types::packet_data::unescrow::IbcTransferUnescrowPayloadData;

pub enum IbcTransferPayloadData<Chain, Counterparty>
where
    Chain: HasAmountType,
    Counterparty: HasAmountType + HasAddressType,
{
    Mint(IbcTransferMintPayloadData<Chain, Counterparty>),
    Unescrow(IbcTransferUnescrowPayloadData<Chain, Counterparty>),
}

pub struct UseIbcTransferPayloadData;

#[cgp_provider(PayloadDataTypeComponent)]
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
        match self {
            Self::Mint(payload) => Self::Mint(payload.clone()),
            Self::Unescrow(payload) => Self::Unescrow(payload.clone()),
        }
    }
}

impl<Chain, Counterparty> PartialEq for IbcTransferPayloadData<Chain, Counterparty>
where
    Chain: HasAmountType<Amount: Eq>,
    Counterparty: HasAmountType<Amount: Eq> + HasAddressType<Address: Eq>,
{
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Mint(data), Self::Mint(other)) => data == other,
            (Self::Unescrow(data), Self::Unescrow(other)) => data == other,
            _ => false,
        }
    }
}

impl<Chain, Counterparty> Eq for IbcTransferPayloadData<Chain, Counterparty>
where
    Chain: HasAmountType<Amount: Eq>,
    Counterparty: HasAmountType<Amount: Eq> + HasAddressType<Address: Eq>,
{
}

impl<Chain, Counterparty> From<IbcTransferMintPayloadData<Chain, Counterparty>>
    for IbcTransferPayloadData<Chain, Counterparty>
where
    Chain: HasAmountType,
    Counterparty: HasAmountType + HasAddressType,
{
    fn from(value: IbcTransferMintPayloadData<Chain, Counterparty>) -> Self {
        Self::Mint(value)
    }
}

impl<Chain, Counterparty> From<IbcTransferUnescrowPayloadData<Chain, Counterparty>>
    for IbcTransferPayloadData<Chain, Counterparty>
where
    Chain: HasAmountType,
    Counterparty: HasAmountType + HasAddressType,
{
    fn from(value: IbcTransferUnescrowPayloadData<Chain, Counterparty>) -> Self {
        Self::Unescrow(value)
    }
}
