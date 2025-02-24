use cgp::prelude::*;
use hermes_cosmos_chain_components::types::payloads::client::CosmosUpdateClientPayload;
use hermes_relayer_components::chain::traits::message_builders::update_client::{
    UpdateClientMessageBuilder, UpdateClientMessageBuilderComponent,
};
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_relayer_components::chain::traits::types::update_client::HasUpdateClientPayloadType;
use ibc::core::host::types::identifiers::ClientId;

use crate::types::message::SolomachineMessage;

pub struct BuildUpdateCosmosClientMessage;

#[cgp_provider(UpdateClientMessageBuilderComponent)]
impl<Chain, Counterparty> UpdateClientMessageBuilder<Chain, Counterparty>
    for BuildUpdateCosmosClientMessage
where
    Chain: HasMessageType<Message = SolomachineMessage>
        + HasIbcChainTypes<Counterparty, ClientId = ClientId>
        + HasAsyncErrorType,
    Counterparty:
        HasUpdateClientPayloadType<Chain, UpdateClientPayload = CosmosUpdateClientPayload>,
{
    async fn build_update_client_message(
        _chain: &Chain,
        _client_id: &ClientId,
        counterparty_payload: CosmosUpdateClientPayload,
    ) -> Result<Vec<SolomachineMessage>, Chain::Error> {
        let message = SolomachineMessage::CosmosUpdateClient(Box::new(counterparty_payload));

        Ok(vec![message])
    }
}
