use core::fmt::Debug;
use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_chain_components::traits::CanExtractFromMessageResponse;
use hermes_chain_type_components::traits::HasMessageResponseEvents;

use crate::chain::traits::{
    CanBuildCreateClientMessage, CanBuildCreateClientPayload, CanSendSingleMessage, HasChainId,
    HasChainIdType, HasCreateClientEvent, HasCreateClientPayloadType,
};
use crate::relay::traits::{
    ClientCreator, ClientCreatorComponent, HasTargetChainTypes, RelayTarget,
};

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

#[cgp_provider(ClientCreatorComponent)]
impl<Relay, Target, TargetChain, CounterpartyChain> ClientCreator<Relay, Target>
    for CreateClientWithChains
where
    Target: RelayTarget,
    Relay: HasTargetChainTypes<
            Target,
            TargetChain = TargetChain,
            CounterpartyChain = CounterpartyChain,
        > + for<'a> CanRaiseAsyncError<
            MissingCreateClientEventError<'a, TargetChain, CounterpartyChain>,
        > + CanRaiseAsyncError<TargetChain::Error>
        + CanRaiseAsyncError<CounterpartyChain::Error>,
    TargetChain: CanSendSingleMessage
        + HasChainId
        + CanBuildCreateClientMessage<CounterpartyChain>
        + HasCreateClientEvent<CounterpartyChain>
        + CanExtractFromMessageResponse<TargetChain::CreateClientEvent>
        + HasMessageResponseEvents,
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
            .map_err(Relay::raise_error)?;

        let message = target_chain
            .build_create_client_message(create_client_message_options, payload)
            .await
            .map_err(Relay::raise_error)?;

        let response = target_chain
            .send_message(message)
            .await
            .map_err(Relay::raise_error)?;

        let create_client_event = target_chain
            .try_extract_from_message_response(PhantomData, &response)
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
