use alloc::vec::Vec;

use hermes_prelude::*;

use crate::chain::traits::CanSendMessages;
use crate::relay::traits::{
    HasTargetChains, IbcMessageSender, IbcMessageSenderComponent, RelayTarget,
};

pub struct SendIbcMessagesToChain;

#[cgp_provider(IbcMessageSenderComponent<Sink>)]
impl<Relay, Sink, Target, TargetChain> IbcMessageSender<Relay, Sink, Target>
    for SendIbcMessagesToChain
where
    Target: RelayTarget,
    Relay:
        HasTargetChains<Target, TargetChain = TargetChain> + CanRaiseAsyncError<TargetChain::Error>,
    TargetChain: CanSendMessages,
{
    async fn send_messages(
        relay: &Relay,
        _target: Target,
        messages: Vec<TargetChain::Message>,
    ) -> Result<Vec<TargetChain::MessageResponse>, Relay::Error> {
        let events = relay
            .target_chain()
            .send_messages(messages)
            .await
            .map_err(Relay::raise_error)?;

        Ok(events)
    }
}
