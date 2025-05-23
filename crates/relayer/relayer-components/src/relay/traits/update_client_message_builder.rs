use alloc::vec::Vec;

use cgp::core::error::ErrorOf;
use hermes_chain_components::traits::{HasHeightType, HasMessageType};
use hermes_prelude::*;

use crate::chain::traits::CanSendMessages;
use crate::chain::types::aliases::{HeightOf, MessageOf};
use crate::relay::traits::{HasTargetChainTypes, HasTargetChains, RelayTarget};

#[cgp_component {
  provider: TargetUpdateClientMessageBuilder,
  context: Relay,
}]
#[async_trait]
pub trait CanBuildTargetUpdateClientMessage<Target: RelayTarget>:
    HasTargetChainTypes<Target, TargetChain: HasMessageType, CounterpartyChain: HasHeightType>
    + HasAsyncErrorType
{
    async fn build_target_update_client_messages(
        &self,
        _target: Target,
        height: &HeightOf<Self::CounterpartyChain>,
    ) -> Result<Vec<MessageOf<Self::TargetChain>>, Self::Error>;
}

#[async_trait]
pub trait CanSendTargetUpdateClientMessage<Target: RelayTarget>:
    HasTargetChainTypes<Target, CounterpartyChain: HasHeightType> + HasAsyncErrorType
{
    async fn send_target_update_client_messages(
        &self,
        target: Target,
        height: &HeightOf<Self::CounterpartyChain>,
    ) -> Result<(), Self::Error>;
}

impl<Relay, Target> CanSendTargetUpdateClientMessage<Target> for Relay
where
    Relay: HasTargetChains<Target>
        + CanBuildTargetUpdateClientMessage<Target>
        + CanRaiseAsyncError<ErrorOf<Relay::TargetChain>>,
    Target: RelayTarget,
    Relay::TargetChain: CanSendMessages,
{
    async fn send_target_update_client_messages(
        &self,
        target: Target,
        height: &HeightOf<Relay::CounterpartyChain>,
    ) -> Result<(), Self::Error> {
        let messages = self
            .build_target_update_client_messages(target, height)
            .await?;

        // If there are no UpdateClient messages returned, it means that the IBC client is
        // already up to date.
        if !messages.is_empty() {
            self.target_chain()
                .send_messages(messages)
                .await
                .map_err(Relay::raise_error)?;
        }

        Ok(())
    }
}
