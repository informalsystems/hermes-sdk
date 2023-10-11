use cgp_core::prelude::*;
use ibc_relayer_components::chain::traits::components::message_sender::MessageSenderComponent;
use ibc_relayer_components::components::default::chain::DefaultChainComponents;

use crate::impls::chain::components::process_message::ProcessSolomachineMessages;
use crate::types::chain::SolomachineChain;

pub struct SolomachineChainComponents;

impl<Chain> HasComponents for SolomachineChain<Chain>
where
    Chain: Async,
{
    type Components = DefaultChainComponents<SolomachineChainComponents>;
}

delegate_components!(
    SolomachineChainComponents;
    MessageSenderComponent:
        ProcessSolomachineMessages,
);
