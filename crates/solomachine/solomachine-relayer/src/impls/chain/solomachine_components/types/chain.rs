use cgp_core::Async;
use hermes_relayer_components::chain::traits::types::event::ProvideEventType;
use hermes_relayer_components::chain::traits::types::message::ProvideMessageType;

use crate::types::event::SolomachineEvent;
use crate::types::message::SolomachineMessage;

pub struct ProvideSolomachineChainTypes;

impl<Chain> ProvideMessageType<Chain> for ProvideSolomachineChainTypes
where
    Chain: Async,
{
    type Message = SolomachineMessage;
}

impl<Chain> ProvideEventType<Chain> for ProvideSolomachineChainTypes
where
    Chain: Async,
{
    type Event = SolomachineEvent;
}
