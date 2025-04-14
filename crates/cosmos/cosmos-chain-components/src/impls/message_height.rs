use cgp::prelude::*;
use hermes_relayer_components::chain::traits::{
    CounterpartyMessageHeightGetter, CounterpartyMessageHeightGetterComponent, HasHeightType,
    HasMessageType,
};
use ibc::core::client::types::Height;

use crate::traits::message::CosmosMessage;

pub struct GetCosmosCounterpartyMessageHeight;

#[cgp_provider(CounterpartyMessageHeightGetterComponent)]
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
