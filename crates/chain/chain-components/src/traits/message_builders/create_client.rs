use cgp::core::component::DelegateTo;
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

impl<Chain, Counterparty, Components, Delegate> CreateClientMessageBuilder<Chain, Counterparty>
    for DelegateTo<Components>
where
    Chain: HasCreateClientMessageOptionsType<Counterparty> + HasMessageType + HasErrorType,
    Counterparty: HasCreateClientPayloadType<Chain>,
    Delegate: CreateClientMessageBuilder<Chain, Counterparty>,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
{
    async fn build_create_client_message(
        chain: &Chain,
        create_client_options: &Chain::CreateClientMessageOptions,
        counterparty_payload: Counterparty::CreateClientPayload,
    ) -> Result<Chain::Message, Chain::Error> {
        Delegate::build_create_client_message(chain, create_client_options, counterparty_payload)
            .await
    }
}
