use hermes_chain_type_components::traits::types::address::HasAddressType;
use hermes_chain_type_components::traits::types::amount::HasAmountType;
use hermes_ibc_components::traits::types::payload::data::ProvidePayloadDataType;

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
