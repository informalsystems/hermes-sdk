use alloc::vec::Vec;

use crate::chain::traits::send_message::CanSendMessages;
use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::relay::traits::chains::HasRelayChains;
use crate::relay::traits::ibc_message_sender::IbcMessageSender;
use crate::relay::traits::target::ChainTarget;

pub struct SendIbcMessagesToChain;

impl<Relay, Sink, Target, TargetChain> IbcMessageSender<Relay, Sink, Target>
    for SendIbcMessagesToChain
where
    Relay: HasRelayChains,
    Target: ChainTarget<Relay, TargetChain = TargetChain>,
    TargetChain: CanSendMessages,
    TargetChain: HasIbcChainTypes<Target::CounterpartyChain>,
{
    async fn send_messages(
        relay: &Relay,
        _target: Target,
        messages: Vec<TargetChain::Message>,
    ) -> Result<Vec<TargetChain::MessageResponse>, Relay::Error> {
        let events = Target::target_chain(relay)
            .send_messages(messages)
            .await
            .map_err(Target::target_chain_error)?;

        Ok(events)
    }
}
