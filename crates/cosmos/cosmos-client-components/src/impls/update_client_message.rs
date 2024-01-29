use cgp_core::HasErrorType;
use hermes_relayer_components::chain::traits::components::update_client_message_builder::UpdateClientMessageBuilder;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::update_client::HasUpdateClientPayloadType;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;

use crate::traits::message::{CosmosMessage, ToCosmosMessage};
use crate::types::messages::client::update::CosmosUpdateClientMessage;
use crate::types::payloads::client::CosmosUpdateClientPayload;

pub struct BuildCosmosUpdateClientMessage;

impl<Chain, Counterparty> UpdateClientMessageBuilder<Chain, Counterparty>
    for BuildCosmosUpdateClientMessage
where
    Chain:
        HasIbcChainTypes<Counterparty, ClientId = ClientId, Message = CosmosMessage> + HasErrorType,
    Counterparty:
        HasUpdateClientPayloadType<Chain, UpdateClientPayload = CosmosUpdateClientPayload>,
{
    async fn build_update_client_message(
        _chain: &Chain,
        client_id: &ClientId,
        payload: CosmosUpdateClientPayload,
    ) -> Result<Vec<CosmosMessage>, Chain::Error> {
        let messages = payload
            .headers
            .into_iter()
            .map(|header| {
                let message = CosmosUpdateClientMessage {
                    client_id: client_id.clone(),
                    header: header.into(),
                };

                message.to_cosmos_message()
            })
            .collect();

        Ok(messages)
    }
}
