use cgp_core::async_trait;

use crate::chain::traits::message_builders::create_client::CanBuildCreateClientMessage;
use crate::chain::traits::payload_builders::create_client::CanBuildCreateClientPayload;
use crate::chain::traits::send_message::CanSendSingleMessage;
use crate::chain::traits::types::create_client::{
    HasCreateClientEvent, HasCreateClientPayloadType,
};
use crate::relay::traits::chains::HasRelayChains;
use crate::relay::traits::client_creator::ClientCreator;
use crate::relay::traits::target::ChainTarget;

pub struct CreateClientWithChains;

pub trait CanRaiseMissingCreateClientEventError<Target>: HasRelayChains
where
    Target: ChainTarget<Self>,
{
    fn missing_create_client_event_error(
        target_chain: &Target::TargetChain,
        counterparty_chain: &Target::CounterpartyChain,
    ) -> Self::Error;
}

#[async_trait]
impl<Relay, Target, TargetChain, CounterpartyChain> ClientCreator<Relay, Target>
    for CreateClientWithChains
where
    Relay: CanRaiseMissingCreateClientEventError<Target>,
    Target: ChainTarget<Relay, TargetChain = TargetChain, CounterpartyChain = CounterpartyChain>,
    TargetChain: CanSendSingleMessage
        + CanBuildCreateClientMessage<CounterpartyChain>
        + HasCreateClientEvent<CounterpartyChain>,
    CounterpartyChain:
        CanBuildCreateClientPayload<TargetChain> + HasCreateClientPayloadType<TargetChain>,
    TargetChain::ClientId: Clone,
{
    async fn create_client(
        _target: Target,
        target_chain: &TargetChain,
        counterparty_chain: &CounterpartyChain,
        create_client_options: &CounterpartyChain::CreateClientOptions,
    ) -> Result<TargetChain::ClientId, Relay::Error> {
        let payload = counterparty_chain
            .build_create_client_payload(create_client_options)
            .await
            .map_err(Target::counterparty_chain_error)?;

        let message = target_chain
            .build_create_client_message(payload)
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
                Relay::missing_create_client_event_error(target_chain, counterparty_chain)
            })?;

        let client_id = TargetChain::create_client_event_client_id(&create_client_event);

        Ok(client_id.clone())
    }
}
