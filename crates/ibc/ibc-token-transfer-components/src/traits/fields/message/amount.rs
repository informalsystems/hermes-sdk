use hermes_chain_type_components::traits::types::amount::HasAmountType;
use hermes_ibc_components::traits::types::message::HasIbcMessageType;

pub trait HasMessageSendTransferAmount<Counterparty, App>:
    HasAmountType + HasIbcMessageType<Counterparty, App>
{
    fn message_send_transfer_amount(message: &Self::IbcMessage) -> &Self::Amount;
}
