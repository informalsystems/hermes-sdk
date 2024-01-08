use cgp_core::prelude::*;
use cgp_core::HasErrorType;
use hermes_cosmos_client_components::types::payloads::client::CosmosUpdateClientPayload;
use hermes_relayer_components::chain::traits::components::update_client_message_builder::UpdateClientMessageBuilder;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_relayer_components::chain::traits::types::update_client::HasUpdateClientPayload;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;

use crate::types::message::SolomachineMessage;

pub struct BuildUpdateCosmosClientMessage;

#[async_trait]
impl<Chain, Counterparty> UpdateClientMessageBuilder<Chain, Counterparty>
    for BuildUpdateCosmosClientMessage
where
    Chain: HasMessageType<Message = SolomachineMessage>
        + HasIbcChainTypes<Counterparty, ClientId = ClientId>
        + HasErrorType,
    Counterparty: HasUpdateClientPayload<Chain, UpdateClientPayload = CosmosUpdateClientPayload>,
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
