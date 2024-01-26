use cgp_core::HasErrorType;
use hermes_cosmos_client_components::types::payloads::client::CosmosCreateClientPayload;
use hermes_relayer_components::chain::traits::components::create_client_message_builder::CreateClientMessageBuilder;
use hermes_relayer_components::chain::traits::types::create_client::HasCreateClientPayload;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;

use crate::types::message::SovereignMessage;

/**
   Build a message to create a Cosmos client on a Sovereign rollup
*/
pub struct BuildCreateCosmosClientMessageOnSovereign;

impl<Chain, Counterparty> CreateClientMessageBuilder<Chain, Counterparty>
    for BuildCreateCosmosClientMessageOnSovereign
where
    Chain: HasMessageType<Message = SovereignMessage> + HasErrorType,
    Counterparty: HasCreateClientPayload<Chain, CreateClientPayload = CosmosCreateClientPayload>,
{
    async fn build_create_client_message(
        _chain: &Chain,
        _payload: CosmosCreateClientPayload,
    ) -> Result<SovereignMessage, Chain::Error> {
        todo!()
    }
}
