use cgp_core::Async;
use ibc_relayer_components::chain::traits::types::message::MessageTypeProvider;

use crate::types::message::SolomachineMessage;

pub struct ProvideSolomachineChainTypes;

impl<Chain> MessageTypeProvider<Chain> for ProvideSolomachineChainTypes
where
    Chain: Async,
{
    type Message = SolomachineMessage;
}
