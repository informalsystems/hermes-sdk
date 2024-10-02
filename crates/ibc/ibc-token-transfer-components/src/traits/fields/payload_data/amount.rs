use cgp::prelude::*;
use hermes_chain_type_components::traits::types::amount::HasAmountType;
use hermes_ibc_components::traits::types::payload::data::HasPayloadDataType;

pub enum IbcTransferAmount<Chain, Counterparty>
where
    Chain: HasAmountType,
    Counterparty: HasAmountType,
{
    Mint(Chain::Amount),
    Unescrow(Counterparty::Amount),
}

#[derive_component(IbcTransferReceiverGetterComponent, IbcTransferReceiverGetter<Chain>)]
pub trait HasIbcTransferDenom<Counterparty, App>:
    HasAmountType + HasPayloadDataType<Counterparty, App>
where
    Counterparty: HasAmountType,
{
    fn ibc_transfer_denom(
        payload_data: &Self::PayloadData,
    ) -> &IbcTransferAmount<Self, Counterparty>;
}
