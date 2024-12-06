use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::ibc::CounterpartyMessageHeightGetter;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use ibc::core::client::types::Height;

use crate::traits::message::CosmosMessage;

pub struct GetCosmosCounterpartyMessageHeight;

impl<Chain, Counterparty> CounterpartyMessageHeightGetter<Chain, Counterparty>
    for GetCosmosCounterpartyMessageHeight
where
    Chain: HasMessageType<Message = CosmosMessage>,
    Counterparty: HasHeightType<Height = Height>,
{
    fn counterparty_message_height_for_update_client(message: &CosmosMessage) -> Option<Height> {
        message
            .message
            .counterparty_message_height_for_update_client()
    }
}
