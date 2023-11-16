use alloc::sync::Arc;
use async_trait::async_trait;
use cgp_core::DelegateComponent;
use cosmos_client_components::components::create_client_message::BuildCosmosCreateClientMessage;
use cosmos_client_components::traits::message::CosmosMessage;
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer_components::chain::traits::components::create_client_message_builder::CreateClientMessageBuilder;
use ibc_relayer_components::chain::traits::types::create_client::HasCreateClientPayload;

use crate::contexts::chain::CosmosChain;
use crate::types::error::Error;

pub struct DelegateCosmosCreateClientMessageBuilder;

impl<Counterparty> DelegateComponent<CosmosChain<Counterparty>>
    for DelegateCosmosCreateClientMessageBuilder
{
    type Delegate = BuildCosmosCreateClientMessage;
}

#[async_trait]
impl<Chain, Counterparty, Delegate> CreateClientMessageBuilder<CosmosChain<Chain>, Counterparty>
    for DelegateCosmosCreateClientMessageBuilder
where
    Chain: ChainHandle,
    Counterparty: HasCreateClientPayload<CosmosChain<Chain>>,
    Delegate: CreateClientMessageBuilder<CosmosChain<Chain>, Counterparty>,
    Self: DelegateComponent<Counterparty, Delegate = Delegate>,
{
    async fn build_create_client_message(
        chain: &CosmosChain<Chain>,
        counterparty_payload: Counterparty::CreateClientPayload,
    ) -> Result<Arc<dyn CosmosMessage>, Error> {
        Delegate::build_create_client_message(chain, counterparty_payload).await
    }
}
