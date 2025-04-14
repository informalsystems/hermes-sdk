use cgp::prelude::*;
use hermes_cosmos_chain_components::traits::{CosmosMessage, ToCosmosMessage};
use hermes_cosmos_chain_components::types::CosmosUpdateClientMessage;
use hermes_protobuf_encoding_components::types::any::Any;
use hermes_relayer_components::chain::traits::{
    HasIbcChainTypes, HasUpdateClientPayloadType, UpdateClientMessageBuilder,
    UpdateClientMessageBuilderComponent,
};
use ibc::core::host::types::identifiers::ClientId;

use crate::methods::encode::header::encode_header;
use crate::types::payloads::client::SolomachineUpdateClientPayload;

pub struct BuildUpdateSolomachineClientMessage;

#[cgp_provider(UpdateClientMessageBuilderComponent)]
impl<Chain, Counterparty> UpdateClientMessageBuilder<Chain, Counterparty>
    for BuildUpdateSolomachineClientMessage
where
    Chain: HasIbcChainTypes<Counterparty, Message = CosmosMessage, ClientId = ClientId>
        + HasAsyncErrorType,
    Counterparty:
        HasUpdateClientPayloadType<Chain, UpdateClientPayload = SolomachineUpdateClientPayload>,
{
    async fn build_update_client_message(
        _chain: &Chain,
        client_id: &ClientId,
        payload: SolomachineUpdateClientPayload,
    ) -> Result<Vec<CosmosMessage>, Chain::Error> {
        let header = encode_header(&payload.header);

        let message = CosmosUpdateClientMessage {
            client_id: client_id.clone(),
            header: Any {
                type_url: header.type_url,
                value: header.value,
            },
        };

        Ok(vec![message.to_cosmos_message()])
    }
}
