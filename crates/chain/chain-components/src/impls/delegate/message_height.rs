use core::marker::PhantomData;

use cgp::core::component::DelegateComponent;

use crate::traits::types::height::HasHeightType;
use crate::traits::types::ibc::CounterpartyMessageHeightGetter;
use crate::traits::types::message::HasMessageType;

pub struct DelegateCounterpartyMessageHeightGetter<Components>(pub PhantomData<Components>);

impl<Chain, Counterparty, Components, Delegate> CounterpartyMessageHeightGetter<Chain, Counterparty>
    for DelegateCounterpartyMessageHeightGetter<Components>
where
    Chain: HasMessageType,
    Counterparty: HasHeightType,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
    Delegate: CounterpartyMessageHeightGetter<Chain, Counterparty>,
{
    fn counterparty_message_height_for_update_client(
        message: &Chain::Message,
    ) -> Option<Counterparty::Height> {
        Delegate::counterparty_message_height_for_update_client(message)
    }
}
