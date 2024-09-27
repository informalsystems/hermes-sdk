use core::marker::PhantomData;

use cgp::prelude::*;

use crate::traits::message_builders::create_client::CreateClientMessageBuilder;
use crate::traits::types::create_client::{
    HasCreateClientMessageOptionsType, HasCreateClientPayloadType,
    ProvideCreateClientMessageOptionsType,
};
use crate::traits::types::message::HasMessageType;

pub struct DelegateBuildCreateClientMessage<Components>(pub PhantomData<Components>);

impl<Chain, Counterparty, Components, Delegate>
    ProvideCreateClientMessageOptionsType<Chain, Counterparty>
    for DelegateBuildCreateClientMessage<Components>
where
    Chain: Async,
    Delegate: ProvideCreateClientMessageOptionsType<Chain, Counterparty>,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
{
    type CreateClientMessageOptions = Delegate::CreateClientMessageOptions;
}

impl<Chain, Counterparty, Components, Delegate> CreateClientMessageBuilder<Chain, Counterparty>
    for DelegateBuildCreateClientMessage<Components>
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
