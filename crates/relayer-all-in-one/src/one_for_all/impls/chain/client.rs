use cgp_core::async_trait;
use ibc_relayer_components::chain::traits::components::client_state_querier::ClientStateQuerier;
use ibc_relayer_components::chain::traits::components::consensus_state_height_querier::ConsensusStateHeightQuerier;
use ibc_relayer_components::chain::traits::components::create_client_message_builder::CreateClientMessageBuilder;
use ibc_relayer_components::chain::traits::components::create_client_payload_builder::CreateClientPayloadBuilder;
use ibc_relayer_components::chain::traits::components::update_client_message_builder::UpdateClientMessageBuilder;
use ibc_relayer_components::chain::traits::components::update_client_payload_builder::UpdateClientPayloadBuilder;
use ibc_relayer_components::chain::traits::types::client_state::{
    HasClientStateFields, HasClientStateType,
};
use ibc_relayer_components::chain::traits::types::create_client::{
    HasCreateClientEvent, HasCreateClientOptions, HasCreateClientPayload,
};
use ibc_relayer_components::chain::traits::types::update_client::HasUpdateClientPayload;

use crate::one_for_all::traits::chain::{OfaChainTypes, OfaIbcChain};
use crate::one_for_all::types::chain::OfaChainWrapper;
use crate::one_for_all::types::component::OfaComponents;
use crate::std_prelude::*;

impl<Chain, Counterparty> HasCreateClientOptions<OfaChainWrapper<Counterparty>>
    for OfaChainWrapper<Chain>
where
    Chain: OfaChainTypes,
    Counterparty: OfaChainTypes,
{
    type CreateClientPayloadOptions = Chain::CreateClientPayloadOptions;
}

impl<Chain, Counterparty> HasCreateClientPayload<OfaChainWrapper<Counterparty>>
    for OfaChainWrapper<Chain>
where
    Chain: OfaChainTypes,
    Counterparty: OfaChainTypes,
{
    type CreateClientPayload = Chain::CreateClientPayload;
}

impl<Chain, Counterparty> HasCreateClientEvent<OfaChainWrapper<Counterparty>>
    for OfaChainWrapper<Chain>
where
    Chain: OfaIbcChain<Counterparty>,
    Counterparty: OfaChainTypes,
{
    type CreateClientEvent = Chain::CreateClientEvent;

    fn try_extract_create_client_event(event: Self::Event) -> Option<Self::CreateClientEvent> {
        Chain::try_extract_create_client_event(event)
    }

    fn create_client_event_client_id(event: &Self::CreateClientEvent) -> &Self::ClientId {
        Chain::create_client_event_client_id(event)
    }
}

#[async_trait]
impl<Chain, Counterparty>
    CreateClientPayloadBuilder<OfaChainWrapper<Chain>, OfaChainWrapper<Counterparty>>
    for OfaComponents
where
    Chain: OfaIbcChain<Counterparty>,
    Counterparty: OfaChainTypes,
{
    async fn build_create_client_payload(
        chain: &OfaChainWrapper<Chain>,
        create_client_options: &Chain::CreateClientPayloadOptions,
    ) -> Result<Chain::CreateClientPayload, Chain::Error> {
        chain
            .chain
            .build_create_client_payload(create_client_options)
            .await
    }
}

#[async_trait]
impl<Chain, Counterparty>
    CreateClientMessageBuilder<OfaChainWrapper<Chain>, OfaChainWrapper<Counterparty>>
    for OfaComponents
where
    Chain: OfaIbcChain<Counterparty>,
    Counterparty: OfaChainTypes,
{
    async fn build_create_client_message(
        chain: &OfaChainWrapper<Chain>,
        counterparty_payload: Counterparty::CreateClientPayload,
    ) -> Result<Chain::Message, Chain::Error> {
        chain
            .chain
            .build_create_client_message(counterparty_payload)
            .await
    }
}

#[async_trait]
impl<Chain, Counterparty> HasClientStateType<OfaChainWrapper<Counterparty>>
    for OfaChainWrapper<Chain>
where
    Chain: OfaChainTypes,
    Counterparty: OfaChainTypes,
{
    type ClientState = Chain::ClientState;
}

#[async_trait]
impl<Chain, Counterparty> HasUpdateClientPayload<OfaChainWrapper<Counterparty>>
    for OfaChainWrapper<Chain>
where
    Chain: OfaChainTypes,
    Counterparty: OfaChainTypes,
{
    type UpdateClientPayload = Chain::UpdateClientPayload;
}

#[async_trait]
impl<Chain, Counterparty>
    UpdateClientPayloadBuilder<OfaChainWrapper<Chain>, OfaChainWrapper<Counterparty>>
    for OfaComponents
where
    Chain: OfaIbcChain<Counterparty>,
    Counterparty: OfaChainTypes,
{
    async fn build_update_client_payload(
        chain: &OfaChainWrapper<Chain>,
        trusted_height: &Chain::Height,
        target_height: &Chain::Height,
        client_state: Chain::ClientState,
    ) -> Result<Chain::UpdateClientPayload, Chain::Error> {
        chain
            .chain
            .build_update_client_payload(trusted_height, target_height, client_state)
            .await
    }
}

#[async_trait]
impl<Chain, Counterparty>
    UpdateClientMessageBuilder<OfaChainWrapper<Chain>, OfaChainWrapper<Counterparty>>
    for OfaComponents
where
    Chain: OfaIbcChain<Counterparty>,
    Counterparty: OfaChainTypes,
{
    async fn build_update_client_message(
        chain: &OfaChainWrapper<Chain>,
        client_id: &Chain::ClientId,
        payload: Counterparty::UpdateClientPayload,
    ) -> Result<Vec<Chain::Message>, Chain::Error> {
        chain
            .chain
            .build_update_client_message(client_id, payload)
            .await
    }
}

#[async_trait]
impl<Chain, Counterparty>
    ConsensusStateHeightQuerier<OfaChainWrapper<Chain>, OfaChainWrapper<Counterparty>>
    for OfaComponents
where
    Chain: OfaIbcChain<Counterparty>,
    Counterparty: OfaChainTypes,
{
    async fn find_consensus_state_height_before(
        chain: &OfaChainWrapper<Chain>,
        client_id: &Chain::ClientId,
        target_height: &Counterparty::Height,
    ) -> Result<Counterparty::Height, Chain::Error> {
        chain
            .chain
            .find_consensus_state_height_before(client_id, target_height)
            .await
    }
}

impl<Chain, Counterparty> HasClientStateFields<OfaChainWrapper<Counterparty>>
    for OfaChainWrapper<Chain>
where
    Chain: OfaIbcChain<Counterparty>,
    Counterparty: OfaChainTypes,
{
    fn client_state_latest_height(client_state: &Self::ClientState) -> &Self::Height {
        Chain::client_state_latest_height(client_state)
    }
}

#[async_trait]
impl<Chain, Counterparty> ClientStateQuerier<OfaChainWrapper<Chain>, OfaChainWrapper<Counterparty>>
    for OfaComponents
where
    Chain: OfaIbcChain<Counterparty>,
    Counterparty: OfaChainTypes,
{
    async fn query_client_state(
        chain: &OfaChainWrapper<Chain>,
        client_id: &Chain::ClientId,
    ) -> Result<Counterparty::ClientState, Chain::Error> {
        chain.chain.query_client_state(client_id).await
    }
}
