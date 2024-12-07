use core::marker::PhantomData;

use cgp::core::component::WithProvider;
use cgp::core::field::FieldGetter;
use cgp::prelude::*;
use hermes_chain_type_components::traits::types::amount::HasAmountType;
use hermes_ibc_components::traits::types::message::HasIbcMessageType;

#[cgp_component {
  name: MessageTransferAmountGetterComponent,
  provider: MessageTransferAmountGetter,
  context: Chain,
}]
pub trait HasMessageTransferAmount<Counterparty, App>:
    HasAmountType + HasIbcMessageType<Counterparty, App>
{
    fn message_transfer_amount(message: &Self::IbcMessage) -> &Self::Amount;
}

impl<Chain, Counterparty, App, Provider> MessageTransferAmountGetter<Chain, Counterparty, App>
    for WithProvider<Provider>
where
    Chain: HasAmountType + HasIbcMessageType<Counterparty, App>,
    Provider: FieldGetter<Chain::IbcMessage, symbol!("amount"), Value = Chain::Amount>,
{
    fn message_transfer_amount(message: &Chain::IbcMessage) -> &Chain::Amount {
        Provider::get_field(message, PhantomData)
    }
}
