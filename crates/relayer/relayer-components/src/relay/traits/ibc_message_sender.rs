use alloc::vec;
use alloc::vec::Vec;

use cgp::prelude::*;
use hermes_chain_components::traits::send_message::EmptyMessageResponse;
use hermes_chain_type_components::traits::types::message_response::MessageResponseOf;

use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::chain::types::aliases::MessageOf;
use crate::relay::traits::chains::HasRelayChains;
use crate::relay::traits::target::ChainTarget;

pub struct MainSink;

#[derive_component(IbcMessageSenderComponent<Sink>, IbcMessageSender<Relay>)]
#[async_trait]
pub trait CanSendIbcMessages<Sink, Target>: HasRelayChains
where
    Target: ChainTarget<Self>,
{
    async fn send_messages(
        &self,
        target: Target,
        messages: Vec<MessageOf<Target::TargetChain>>,
    ) -> Result<Vec<MessageResponseOf<Target::TargetChain>>, Self::Error>;
}

#[async_trait]
pub trait CanSendSingleIbcMessage<Sink, Target>: HasRelayChains
where
    Target: ChainTarget<Self>,
{
    async fn send_message(
        &self,
        target: Target,
        message: MessageOf<Target::TargetChain>,
    ) -> Result<MessageResponseOf<Target::TargetChain>, Self::Error>;
}

impl<Relay, Sink, Target, TargetChain> CanSendSingleIbcMessage<Sink, Target> for Relay
where
    Relay: CanSendIbcMessages<Sink, Target> + CanRaiseError<EmptyMessageResponse>,
    Target: ChainTarget<Relay, TargetChain = TargetChain>,
    TargetChain: HasIbcChainTypes<Target::CounterpartyChain>,
{
    async fn send_message(
        &self,
        target: Target,
        message: TargetChain::Message,
    ) -> Result<TargetChain::MessageResponse, Relay::Error> {
        let events = self
            .send_messages(target, vec![message])
            .await?
            .into_iter()
            .next()
            .ok_or_else(|| Relay::raise_error(EmptyMessageResponse))?;

        Ok(events)
    }
}
