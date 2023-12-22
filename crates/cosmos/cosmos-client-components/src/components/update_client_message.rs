use alloc::sync::Arc;

use async_trait::async_trait;
use cgp_core::HasErrorType;
use hermes_relayer_components::chain::traits::components::update_client_message_builder::UpdateClientMessageBuilder;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::update_client::HasUpdateClientPayload;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;

use crate::traits::message::{CosmosMessage, ToCosmosMessage};
use crate::types::messages::client::update::CosmosUpdateClientMessage;
use crate::types::payloads::client::CosmosUpdateClientPayload;

pub struct BuildCosmosUpdateClientMessage;

#[async_trait]
impl<Chain, Counterparty> UpdateClientMessageBuilder<Chain, Counterparty>
    for BuildCosmosUpdateClientMessage
where
    Chain: HasIbcChainTypes<Counterparty, ClientId = ClientId, Message = Arc<dyn CosmosMessage>>
        + HasErrorType,
    Counterparty: HasUpdateClientPayload<Chain, UpdateClientPayload = CosmosUpdateClientPayload>,
{
    async fn build_update_client_message(
        _chain: &Chain,
        client_id: &Chain::ClientId,
        payload: Counterparty::UpdateClientPayload,
    ) -> Result<Vec<Chain::Message>, Chain::Error> {
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
