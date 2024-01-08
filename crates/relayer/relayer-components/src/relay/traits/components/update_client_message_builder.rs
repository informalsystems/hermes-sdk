use alloc::vec::Vec;

use cgp_core::prelude::*;

use crate::chain::traits::components::message_sender::CanSendMessages;
use crate::chain::types::aliases::{Height, Message};
use crate::relay::traits::chains::HasRelayChains;
use crate::relay::traits::target::ChainTarget;

#[derive_component(UpdateClientMessageBuilderComponent, UpdateClientMessageBuilder<Relay>)]
#[async_trait]
pub trait CanBuildUpdateClientMessage<Target>: HasRelayChains
where
    Target: ChainTarget<Self>,
{
    async fn build_update_client_messages(
        &self,
        _target: Target,
        height: &Height<Target::CounterpartyChain>,
    ) -> Result<Vec<Message<Target::TargetChain>>, Self::Error>;
}

#[async_trait]
pub trait CanSendUpdateClientMessage<Target>: HasRelayChains
where
    Target: ChainTarget<Self>,
{
    async fn send_update_client_messages(
        &self,
        target: Target,
        height: &Height<Target::CounterpartyChain>,
    ) -> Result<(), Self::Error>;
}

#[async_trait]
impl<Relay, Target> CanSendUpdateClientMessage<Target> for Relay
where
    Relay: CanBuildUpdateClientMessage<Target>,
    Target: ChainTarget<Relay>,
    Target::TargetChain: CanSendMessages,
{
    async fn send_update_client_messages(
        &self,
        target: Target,
        height: &Height<Target::CounterpartyChain>,
    ) -> Result<(), Self::Error> {
        let messages = self.build_update_client_messages(target, height).await?;

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
