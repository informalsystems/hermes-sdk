use cgp_core::prelude::*;
use hermes_cosmos_chain_components::components::ibc_client::CosmosIbcClientComponents;
use hermes_cosmos_chain_components::traits::message::CosmosMessage;
use hermes_relayer_components::chain::traits::message_builders::update_client::UpdateClientMessageBuilder;
use hermes_relayer_components::chain::traits::types::update_client::HasUpdateClientPayloadType;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;

use crate::contexts::chain::CosmosChain;
use crate::types::error::Error;

pub struct DelegateCosmosUpdateClientMessageBuilder;

delegate_components! {
    DelegateCosmosUpdateClientMessageBuilder {
        CosmosChain: CosmosIbcClientComponents,
    }
}

#[async_trait]
impl<Counterparty, Delegate> UpdateClientMessageBuilder<CosmosChain, Counterparty>
    for DelegateCosmosUpdateClientMessageBuilder
where
    Counterparty: HasUpdateClientPayloadType<CosmosChain>,
    Delegate: UpdateClientMessageBuilder<CosmosChain, Counterparty>,
    Self: DelegateComponent<Counterparty, Delegate = Delegate>,
{
    async fn build_update_client_message(
        chain: &CosmosChain,
        client_id: &ClientId,
        payload: Counterparty::UpdateClientPayload,
    ) -> Result<Vec<CosmosMessage>, Error> {
        Delegate::build_update_client_message(chain, client_id, payload).await
    }
}
