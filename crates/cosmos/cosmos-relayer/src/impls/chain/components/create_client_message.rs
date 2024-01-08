use alloc::sync::Arc;

use async_trait::async_trait;
use cgp_core::DelegateComponent;
use hermes_cosmos_client_components::components::create_client_message::BuildCosmosCreateClientMessage;
use hermes_cosmos_client_components::traits::message::CosmosMessage;
use hermes_relayer_components::chain::traits::components::create_client_message_builder::CreateClientMessageBuilder;
use hermes_relayer_components::chain::traits::types::create_client::HasCreateClientPayload;

use crate::contexts::chain::CosmosChain;
use crate::types::error::Error;

pub struct DelegateCosmosCreateClientMessageBuilder;

impl DelegateComponent<CosmosChain> for DelegateCosmosCreateClientMessageBuilder {
    type Delegate = BuildCosmosCreateClientMessage;
}

#[async_trait]
impl<Counterparty, Delegate> CreateClientMessageBuilder<CosmosChain, Counterparty>
    for DelegateCosmosCreateClientMessageBuilder
where
    Counterparty: HasCreateClientPayload<CosmosChain>,
    Delegate: CreateClientMessageBuilder<CosmosChain, Counterparty>,
    Self: DelegateComponent<Counterparty, Delegate = Delegate>,
{
    async fn build_create_client_message(
        chain: &CosmosChain,
        counterparty_payload: Counterparty::CreateClientPayload,
    ) -> Result<CosmosMessage, Error> {
        Delegate::build_create_client_message(chain, counterparty_payload).await
    }
}
