use cgp_core::Async;
use ibc_relayer_components::chain::traits::types::event::EventTypeProvider;
use ibc_relayer_components::chain::traits::types::message::MessageTypeProvider;

use crate::types::event::SolomachineEvent;
use crate::types::message::SolomachineMessage;

pub struct ProvideSolomachineChainTypes;

impl<Chain> MessageTypeProvider<Chain> for ProvideSolomachineChainTypes
where
    Chain: Async,
{
    type Message = SolomachineMessage;
}

impl<Chain> EventTypeProvider<Chain> for ProvideSolomachineChainTypes
where
    Chain: Async,
{
    type Event = SolomachineEvent;
}
