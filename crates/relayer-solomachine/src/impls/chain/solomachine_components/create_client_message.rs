use async_trait::async_trait;
use cgp_core::HasErrorType;
use ibc_relayer_components::chain::traits::components::create_client_message_builder::CreateClientMessageBuilder;
use ibc_relayer_components::chain::traits::types::create_client::HasCreateClientPayload;
use ibc_relayer_components::chain::traits::types::message::HasMessageType;
use ibc_relayer_cosmos::types::payloads::client::CosmosCreateClientPayload;

use crate::types::message::SolomachineMessage;

pub struct BuildCreateCosmosClientMessage;

#[async_trait]
impl<Chain, Counterparty> CreateClientMessageBuilder<Chain, Counterparty>
    for BuildCreateCosmosClientMessage
where
    Chain: HasMessageType<Message = SolomachineMessage> + HasErrorType,
    Counterparty: HasCreateClientPayload<Chain, CreateClientPayload = CosmosCreateClientPayload>,
{
    async fn build_create_client_message(
        _chain: &Chain,
        counterparty_payload: CosmosCreateClientPayload,
    ) -> Result<SolomachineMessage, Chain::Error> {
        let message: SolomachineMessage =
            SolomachineMessage::CosmosCreateClient(Box::new(counterparty_payload));

        Ok(message)
    }
}
