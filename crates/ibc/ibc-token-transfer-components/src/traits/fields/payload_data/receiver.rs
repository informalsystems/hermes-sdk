use cgp::prelude::*;
use hermes_chain_type_components::traits::types::address::HasAddressType;
use hermes_ibc_components::traits::types::payload::data::HasPayloadDataType;

#[derive_component(IbcTransferReceiverGetterComponent, IbcTransferReceiverGetter<Chain>)]
pub trait HasIbcTransferReceiver<Counterparty, App>: HasPayloadDataType<Counterparty, App>
where
    Counterparty: HasAddressType,
{
    fn ibc_transfer_receiver(payload_data: &Self::PayloadData) -> &Counterparty::Address;
}
