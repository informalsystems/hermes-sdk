use alloc::vec::Vec;

use cgp_core::prelude::*;

use crate::chain::traits::send_message::CanSendMessages;
use crate::chain::types::aliases::{HeightOf, MessageOf};
use crate::relay::traits::chains::HasRelayChains;
use crate::relay::traits::target::ChainTarget;

#[derive_component(TargetUpdateClientMessageBuilderComponent, TargetUpdateClientMessageBuilder<Relay>)]
#[async_trait]
pub trait CanBuildTargetUpdateClientMessage<Target>: HasRelayChains
where
    Target: ChainTarget<Self>,
{
    async fn build_target_update_client_messages(
        &self,
        _target: Target,
        height: &HeightOf<Target::CounterpartyChain>,
    ) -> Result<Vec<MessageOf<Target::TargetChain>>, Self::Error>;
}

#[async_trait]
pub trait CanSendTargetUpdateClientMessage<Target>: HasRelayChains
where
    Target: ChainTarget<Self>,
{
    async fn send_target_update_client_messages(
        &self,
        target: Target,
        height: &HeightOf<Target::CounterpartyChain>,
    ) -> Result<(), Self::Error>;
}

#[async_trait]
impl<Relay, Target> CanSendTargetUpdateClientMessage<Target> for Relay
where
    Relay: CanBuildTargetUpdateClientMessage<Target>,
    Target: ChainTarget<Relay>,
    Target::TargetChain: CanSendMessages,
{
    async fn send_target_update_client_messages(
        &self,
        target: Target,
        height: &HeightOf<Target::CounterpartyChain>,
    ) -> Result<(), Self::Error> {
        let messages = self
            .build_target_update_client_messages(target, height)
            .await?;

        // If there are no UpdateClient messages returned, it means that the IBC client is
        // already up to date.
        if !messages.is_empty() {
            Target::target_chain(self)
                .send_messages(messages)
                .await
                .map_err(Target::target_chain_error)?;
        }

        Ok(())
    }
}
