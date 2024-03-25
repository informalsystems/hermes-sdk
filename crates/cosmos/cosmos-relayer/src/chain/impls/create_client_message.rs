use cgp_core::prelude::*;
use hermes_cosmos_chain_components::components::ibc_client::CosmosIbcClientComponents;
use hermes_cosmos_chain_components::traits::message::CosmosMessage;
use hermes_relayer_components::chain::traits::message_builders::create_client::CreateClientMessageBuilder;
use hermes_relayer_components::chain::traits::types::create_client::HasCreateClientPayloadType;

use crate::contexts::chain::CosmosChain;
use crate::types::error::Error;

pub struct DelegateCosmosCreateClientMessageBuilder;

delegate_components! {
    DelegateCosmosCreateClientMessageBuilder {
        CosmosChain: CosmosIbcClientComponents,
    }
}

impl<Counterparty, Delegate> CreateClientMessageBuilder<CosmosChain, Counterparty>
    for DelegateCosmosCreateClientMessageBuilder
where
    Counterparty: HasCreateClientPayloadType<CosmosChain>,
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
