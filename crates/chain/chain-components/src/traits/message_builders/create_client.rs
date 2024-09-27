use cgp::prelude::*;

use crate::traits::types::create_client::{
    HasCreateClientMessageOptionsType, HasCreateClientPayloadType,
};
use crate::traits::types::message::HasMessageType;

#[derive_component(CreateClientMessageBuilderComponent, CreateClientMessageBuilder<Chain>)]
#[async_trait]
pub trait CanBuildCreateClientMessage<Counterparty>:
    HasCreateClientMessageOptionsType<Counterparty> + HasMessageType + HasErrorType
where
    Counterparty: HasCreateClientPayloadType<Self>,
{
    async fn build_create_client_message(
        &self,
        create_client_options: &Self::CreateClientMessageOptions,
        counterparty_payload: Counterparty::CreateClientPayload,
    ) -> Result<Self::Message, Self::Error>;
}
