use cgp_core::prelude::*;

use crate::chain::traits::types::create_client::HasCreateClientPayloadType;
use crate::chain::traits::types::message::HasMessageType;

#[derive_component(CreateClientMessageBuilderComponent, CreateClientMessageBuilder<Chain>)]
#[async_trait]
pub trait CanBuildCreateClientMessage<Counterparty>: HasMessageType + HasErrorType
where
    Counterparty: HasCreateClientPayloadType<Self>,
{
    async fn build_create_client_message(
        &self,
        counterparty_payload: Counterparty::CreateClientPayload,
    ) -> Result<Self::Message, Self::Error>;
}
