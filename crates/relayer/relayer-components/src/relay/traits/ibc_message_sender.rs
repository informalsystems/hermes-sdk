use alloc::vec;
use alloc::vec::Vec;

use cgp::prelude::*;
use hermes_chain_components::traits::{EmptyMessageResponse, HasMessageType};
use hermes_chain_type_components::traits::{HasMessageResponseType, MessageResponseOf};

use crate::chain::traits::HasIbcChainTypes;
use crate::chain::types::aliases::MessageOf;
use crate::relay::traits::{HasTargetChainTypes, RelayTarget};

pub struct MainSink;

#[cgp_component {
  name: IbcMessageSenderComponent<Sink>,
  provider: IbcMessageSender,
  context: Relay,
}]
#[async_trait]
pub trait CanSendIbcMessages<Sink, Target: RelayTarget>:
    HasTargetChainTypes<Target, TargetChain: HasMessageType + HasMessageResponseType>
    + HasAsyncErrorType
{
    async fn send_messages(
        &self,
        target: Target,
        messages: Vec<MessageOf<Self::TargetChain>>,
    ) -> Result<Vec<MessageResponseOf<Self::TargetChain>>, Self::Error>;
}

#[async_trait]
pub trait CanSendSingleIbcMessage<Sink, Target: RelayTarget>:
    HasTargetChainTypes<Target, TargetChain: HasMessageType + HasMessageResponseType>
    + HasAsyncErrorType
{
    async fn send_message(
        &self,
        target: Target,
        message: MessageOf<Self::TargetChain>,
    ) -> Result<MessageResponseOf<Self::TargetChain>, Self::Error>;
}

impl<Relay, Sink, Target, TargetChain> CanSendSingleIbcMessage<Sink, Target> for Relay
where
    Relay: HasTargetChainTypes<Target, TargetChain = TargetChain>
        + CanSendIbcMessages<Sink, Target>
        + CanRaiseAsyncError<EmptyMessageResponse>,
    Target: RelayTarget,
    TargetChain: HasIbcChainTypes<Relay::CounterpartyChain> + HasAsyncErrorType,
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
