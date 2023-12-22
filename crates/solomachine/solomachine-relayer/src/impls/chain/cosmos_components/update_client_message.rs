use alloc::sync::Arc;
use async_trait::async_trait;
use cgp_core::HasErrorType;
use hermes_cosmos_client_components::traits::message::{CosmosMessage, ToCosmosMessage};
use hermes_cosmos_client_components::types::messages::client::update::CosmosUpdateClientMessage;
use hermes_cosmos_relayer::types::error::{BaseError, Error};
use hermes_relayer_components::chain::traits::components::update_client_message_builder::UpdateClientMessageBuilder;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::update_client::HasUpdateClientPayload;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;

use crate::methods::encode::header::encode_header;
use crate::types::payloads::client::SolomachineUpdateClientPayload;

pub struct BuildUpdateSolomachineClientMessage;

#[async_trait]
impl<Chain, Counterparty> UpdateClientMessageBuilder<Chain, Counterparty>
    for BuildUpdateSolomachineClientMessage
where
    Chain: HasIbcChainTypes<Counterparty, Message = Arc<dyn CosmosMessage>, ClientId = ClientId>
        + HasErrorType<Error = Error>,
    Counterparty:
        HasUpdateClientPayload<Chain, UpdateClientPayload = SolomachineUpdateClientPayload>,
{
    async fn build_update_client_message(
        _chain: &Chain,
        client_id: &ClientId,
        payload: SolomachineUpdateClientPayload,
    ) -> Result<Vec<Arc<dyn CosmosMessage>>, Error> {
        let header = encode_header(&payload.header).map_err(BaseError::encode)?;

        let message = CosmosUpdateClientMessage {
            client_id: client_id.clone(),
            header,
        };

        Ok(vec![message.to_cosmos_message()])
    }
}
