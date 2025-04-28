use core::marker::PhantomData;

use cgp::core::component::WithProvider;
use cgp::core::field::FieldGetter;
use hermes_chain_type_components::traits::HasAddressType;
use hermes_ibc_components::traits::types::message::HasIbcMessageType;
use hermes_prelude::*;

#[cgp_component {
  provider: MessageTransferAddressGetter,
  context: Chain,
}]
pub trait HasMessageTransferReceiver<Counterparty, App>:
    HasIbcMessageType<Counterparty, App>
where
    Counterparty: HasAddressType,
{
    fn message_transfer_receiver(message: &Self::IbcMessage) -> &Counterparty::Address;
}

#[cgp_provider(MessageTransferAddressGetterComponent)]
impl<Chain, Counterparty, App, Provider> MessageTransferAddressGetter<Chain, Counterparty, App>
    for WithProvider<Provider>
where
    Chain: HasIbcMessageType<Counterparty, App>,
    Counterparty: HasAddressType,
    Provider: FieldGetter<Chain::IbcMessage, symbol!("receiver"), Value = Counterparty::Address>,
{
    fn message_transfer_receiver(message: &Chain::IbcMessage) -> &Counterparty::Address {
        Provider::get_field(message, PhantomData)
    }
}
