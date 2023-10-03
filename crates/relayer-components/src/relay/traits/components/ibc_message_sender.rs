use cgp_async::async_trait;
use cgp_core::traits::Async;
use cgp_macros::derive_component;

use crate::chain::traits::components::message_sender::InjectMismatchIbcEventsCountError;
use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::chain::types::aliases::{Event, Message};
use crate::relay::traits::chains::HasRelayChains;
use crate::relay::traits::target::ChainTarget;
use crate::std_prelude::*;

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
        messages: Vec<Message<Target::TargetChain>>,
    ) -> Result<Vec<Vec<Event<Target::TargetChain>>>, Self::Error>;
}

#[async_trait]
pub trait CanSendFixSizedIbcMessages<Sink, Target>: HasRelayChains
where
    Target: ChainTarget<Self>,
{
    async fn send_messages_fixed<const COUNT: usize>(
        &self,
        target: Target,
        messages: [Message<Target::TargetChain>; COUNT],
    ) -> Result<[Vec<Event<Target::TargetChain>>; COUNT], Self::Error>;
}

#[async_trait]
pub trait CanSendSingleIbcMessage<Sink, Target>: HasRelayChains
where
    Target: ChainTarget<Self>,
{
    async fn send_message(
        &self,
        target: Target,
        message: Message<Target::TargetChain>,
    ) -> Result<Vec<Event<Target::TargetChain>>, Self::Error>;
}

#[async_trait]
impl<Relay, Sink, Target, TargetChain, Event, Message> CanSendFixSizedIbcMessages<Sink, Target>
    for Relay
where
    Relay: CanSendIbcMessages<Sink, Target> + InjectMismatchIbcEventsCountError,
    Target: ChainTarget<Relay, TargetChain = TargetChain>,
    TargetChain: HasIbcChainTypes<Target::CounterpartyChain, Event = Event, Message = Message>,
    Message: Async,
{
    async fn send_messages_fixed<const COUNT: usize>(
        &self,
        target: Target,
        messages: [Message; COUNT],
    ) -> Result<[Vec<Event>; COUNT], Relay::Error> {
        let events_vec = self.send_messages(target, messages.into()).await?;

        let events = events_vec
            .try_into()
            .map_err(|e: Vec<_>| Relay::mismatch_ibc_events_count_error(COUNT, e.len()))?;

        Ok(events)
    }
}

#[async_trait]
impl<Relay, Sink, Target, TargetChain, Event, Message> CanSendSingleIbcMessage<Sink, Target>
    for Relay
where
    Relay: CanSendIbcMessages<Sink, Target>,
    Target: ChainTarget<Relay, TargetChain = TargetChain>,
    TargetChain: HasIbcChainTypes<Target::CounterpartyChain, Event = Event, Message = Message>,
    Message: Async,
{
    async fn send_message(
        &self,
        target: Target,
        message: Message,
    ) -> Result<Vec<Event>, Relay::Error> {
        let events = self
            .send_messages(target, vec![message])
            .await?
            .into_iter()
            .flatten()
            .collect();

        Ok(events)
    }
}
