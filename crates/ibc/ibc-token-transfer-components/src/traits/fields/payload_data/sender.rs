use cgp::prelude::*;
use hermes_chain_type_components::traits::types::address::HasAddressType;
use hermes_ibc_components::traits::types::payload::data::HasPayloadDataType;

#[derive_component(IbcTransferSenderGetterComponent, IbcTransferSenderGetter<Chain>)]
pub trait HasIbcTransferSender<Counterparty, App>:
    HasAddressType + HasPayloadDataType<Counterparty, App>
{
    fn ibc_transfer_sender(payload_data: &Self::PayloadData) -> &Self::Address;
}
