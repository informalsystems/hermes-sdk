use core::marker::PhantomData;

use cgp::core::component::WithProvider;
use cgp::core::field::FieldGetter;
use cgp::prelude::*;
use hermes_chain_type_components::traits::types::address::HasAddressType;
use hermes_ibc_components::traits::types::message::HasIbcMessageType;

#[derive_component(MessageTransferAddressGetterComponent, MessageTransferAddressGetter<Chain>)]
pub trait HasMessageTransferReceiver<Counterparty, App>:
    HasIbcMessageType<Counterparty, App>
where
    Counterparty: HasAddressType,
{
    fn message_transfer_receiver(message: &Self::IbcMessage) -> &Counterparty::Address;
}

impl<Chain, Counterparty, App, Provider> MessageTransferAddressGetter<Chain, Counterparty, App>
    for WithProvider<Provider>
where
    Chain: HasIbcMessageType<Counterparty, App>,
    Counterparty: HasAddressType,
    Provider:
        FieldGetter<Chain::IbcMessage, symbol!("transfer_amount"), Field = Counterparty::Address>,
{
    fn message_transfer_receiver(message: &Chain::IbcMessage) -> &Counterparty::Address {
        Provider::get_field(message, PhantomData)
    }
}
