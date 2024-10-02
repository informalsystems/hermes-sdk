use cgp::prelude::*;
use hermes_chain_type_components::traits::types::address::HasAddressType;
use hermes_chain_type_components::traits::types::denom::HasDenomType;
use hermes_ibc_components::traits::types::payload::data::HasPayloadDataType;

pub enum IbcTransferDenom<Chain, Counterparty>
where
    Chain: HasDenomType,
    Counterparty: HasDenomType,
{
    Mint(Chain::Denom),
    Unescrow(Counterparty::Denom),
}

#[derive_component(IbcTransferReceiverGetterComponent, IbcTransferReceiverGetter<Chain>)]
pub trait HasIbcTransferDenom<Counterparty, App>:
    HasDenomType + HasPayloadDataType<Counterparty, App>
where
    Counterparty: HasDenomType,
{
    fn ibc_transfer_denom(
        payload_data: &Self::PayloadData,
    ) -> &IbcTransferDenom<Self, Counterparty>;
}
