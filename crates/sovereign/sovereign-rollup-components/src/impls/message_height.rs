use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::ibc::CounterpartyMessageHeightGetter;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use ibc_relayer_types::Height;

use crate::types::message::SovereignMessage;
use crate::types::messages::ibc::IbcMessage;

pub struct GetCosmosHeightFromSovereignMessage;

impl<Chain, Counterparty> CounterpartyMessageHeightGetter<Chain, Counterparty>
    for GetCosmosHeightFromSovereignMessage
where
    Chain: HasMessageType<Message = SovereignMessage>,
    Counterparty: HasHeightType<Height = Height>,
{
    fn counterparty_message_height_for_update_client(message: &SovereignMessage) -> Option<Height> {
        match message {
            SovereignMessage::Ibc(IbcMessage::Core(message)) => message.counterparty_height,
            SovereignMessage::Ibc(IbcMessage::Transfer(message)) => {
                Some(message.counterparty_height)
            }
            _ => None,
        }
    }
}
