use cgp_core::error::HasErrorType;
use hermes_cosmos_chain_components::traits::message::{CosmosMessage, ToCosmosMessage};
use hermes_cosmos_chain_components::types::messages::client::update::CosmosUpdateClientMessage;
use hermes_cosmos_relayer::types::error::Error;
use hermes_relayer_components::chain::traits::message_builders::update_client::UpdateClientMessageBuilder;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::chain::traits::types::update_client::HasUpdateClientPayloadType;
use ibc_relayer_types::core::ics24_host::identifier::ClientId;

use crate::methods::encode::header::encode_header;
use crate::types::payloads::client::SolomachineUpdateClientPayload;

pub struct BuildUpdateSolomachineClientMessage;

impl<Chain, Counterparty> UpdateClientMessageBuilder<Chain, Counterparty>
    for BuildUpdateSolomachineClientMessage
where
    Chain: HasIbcChainTypes<Counterparty, Message = CosmosMessage, ClientId = ClientId>
        + HasErrorType<Error = Error>,
    Counterparty:
        HasUpdateClientPayloadType<Chain, UpdateClientPayload = SolomachineUpdateClientPayload>,
{
    async fn build_update_client_message(
        _chain: &Chain,
        client_id: &ClientId,
        payload: SolomachineUpdateClientPayload,
    ) -> Result<Vec<CosmosMessage>, Error> {
        let header = encode_header(&payload.header);

        let message = CosmosUpdateClientMessage {
            client_id: client_id.clone(),
            header,
        };

        Ok(vec![message.to_cosmos_message()])
    }
}
