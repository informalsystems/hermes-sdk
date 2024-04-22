use cgp_core::HasErrorType;
use hermes_cosmos_chain_components::types::payloads::client::CosmosCreateClientPayload;
use hermes_encoding_components::traits::has_encoding::HasEncoding;
use hermes_relayer_components::chain::traits::message_builders::create_client::CreateClientMessageBuilder;
use hermes_relayer_components::chain::traits::types::create_client::HasCreateClientPayloadType;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;

use crate::types::message::SovereignMessage;

/**
   Build a message to create a Cosmos client on a Sovereign rollup
*/
pub struct BuildCreateCosmosClientMessageOnSovereign;

impl<Chain, Counterparty, Encoding> CreateClientMessageBuilder<Chain, Counterparty>
    for BuildCreateCosmosClientMessageOnSovereign
where
    Chain: HasMessageType<Message = SovereignMessage>
        + HasEncoding<Encoding = Encoding>
        + HasErrorType,
    Counterparty:
        HasCreateClientPayloadType<Chain, CreateClientPayload = CosmosCreateClientPayload>,
{
    async fn build_create_client_message(
        _chain: &Chain,
        _payload: CosmosCreateClientPayload,
    ) -> Result<SovereignMessage, Chain::Error> {
        todo!()
    }
}
