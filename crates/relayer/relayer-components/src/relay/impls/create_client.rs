use core::fmt::Debug;

use cgp_core::error::CanRaiseError;

use crate::chain::traits::message_builders::create_client::CanBuildCreateClientMessage;
use crate::chain::traits::payload_builders::create_client::CanBuildCreateClientPayload;
use crate::chain::traits::send_message::CanSendSingleMessage;
use crate::chain::traits::types::chain_id::{HasChainId, HasChainIdType};
use crate::chain::traits::types::create_client::{
    HasCreateClientEvent, HasCreateClientPayloadType,
};
use crate::relay::traits::chains::HasRelayChains;
use crate::relay::traits::client_creator::ClientCreator;
use crate::relay::traits::target::ChainTarget;

pub struct CreateClientWithChains;

pub struct MissingCreateClientEventError<'a, TargetChain, CounterpartyChain>
where
    TargetChain: HasChainIdType,
    CounterpartyChain: HasChainIdType,
{
    pub target_chain_id: &'a TargetChain::ChainId,
    pub counterparty_chain_id: &'a CounterpartyChain::ChainId,
}

impl<'a, TargetChain, CounterpartyChain> Debug
    for MissingCreateClientEventError<'a, TargetChain, CounterpartyChain>
where
    TargetChain: HasChainIdType,
    CounterpartyChain: HasChainIdType,
    TargetChain::ChainId: Debug,
    CounterpartyChain::ChainId: Debug,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("MissingCreateClientEventError")
            .field(
                "message",
                &"missing CreateClient event when creating client",
            )
            .field("target_chain_id", &self.target_chain_id)
            .field("counterparty_chain_id", &self.counterparty_chain_id)
            .finish()
    }
}

impl<Relay, Target, TargetChain, CounterpartyChain> ClientCreator<Relay, Target>
    for CreateClientWithChains
where
    Relay: HasRelayChains
        + for<'a> CanRaiseError<MissingCreateClientEventError<'a, TargetChain, CounterpartyChain>>,
    Target: ChainTarget<Relay, TargetChain = TargetChain, CounterpartyChain = CounterpartyChain>,
    TargetChain: CanSendSingleMessage
        + HasChainId
        + CanBuildCreateClientMessage<CounterpartyChain>
        + HasCreateClientEvent<CounterpartyChain>,
    CounterpartyChain: HasChainId
        + CanBuildCreateClientPayload<TargetChain>
        + HasCreateClientPayloadType<TargetChain>,
    TargetChain::ClientId: Clone,
{
    async fn create_client(
        _target: Target,
        target_chain: &TargetChain,
        counterparty_chain: &CounterpartyChain,
        create_client_payload_options: &CounterpartyChain::CreateClientPayloadOptions,
        create_client_message_options: &TargetChain::CreateClientMessageOptions,
    ) -> Result<TargetChain::ClientId, Relay::Error> {
        let payload = counterparty_chain
            .build_create_client_payload(create_client_payload_options)
            .await
            .map_err(Target::counterparty_chain_error)?;

        let message = target_chain
            .build_create_client_message(create_client_message_options, payload)
            .await
            .map_err(Target::target_chain_error)?;

        let events = target_chain
            .send_message(message)
            .await
            .map_err(Target::target_chain_error)?;

        let create_client_event = events
            .into_iter()
            .find_map(|event| TargetChain::try_extract_create_client_event(event))
            .ok_or_else(|| {
                Relay::raise_error(MissingCreateClientEventError {
                    target_chain_id: target_chain.chain_id(),
                    counterparty_chain_id: counterparty_chain.chain_id(),
                })
            })?;

        let client_id = TargetChain::create_client_event_client_id(&create_client_event);

        Ok(client_id.clone())
    }
}
