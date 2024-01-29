use cgp_core::HasErrorType;
use hermes_cosmos_client_components::traits::message::CosmosMessage;
use hermes_relayer_components::chain::traits::components::create_client_message_builder::CreateClientMessageBuilder;
use hermes_relayer_components::chain::traits::types::create_client::HasCreateClientPayloadType;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;

use crate::types::payloads::client::SovereignCreateClientPayload;

/**
   Build a message to create a Sovereign client on a Cosmos chain
*/
pub struct BuildCreateSovereignClientMessageOnCosmos;

impl<Chain, Counterparty> CreateClientMessageBuilder<Chain, Counterparty>
    for BuildCreateSovereignClientMessageOnCosmos
where
    Chain: HasMessageType<Message = CosmosMessage> + HasErrorType,
    Counterparty:
        HasCreateClientPayloadType<Chain, CreateClientPayload = SovereignCreateClientPayload>,
{
    async fn build_create_client_message(
        _chain: &Chain,
        _payload: SovereignCreateClientPayload,
    ) -> Result<CosmosMessage, Chain::Error> {
        todo!()
    }
}
