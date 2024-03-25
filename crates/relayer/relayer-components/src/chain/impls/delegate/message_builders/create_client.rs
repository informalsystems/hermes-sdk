use core::marker::PhantomData;

use cgp_core::{DelegateComponent, HasErrorType};

use crate::chain::traits::message_builders::create_client::CreateClientMessageBuilder;
use crate::chain::traits::types::create_client::HasCreateClientPayloadType;
use crate::chain::traits::types::message::HasMessageType;

pub struct DelegateBuildCreateClientMessage<Components>(pub PhantomData<Components>);

impl<Chain, Counterparty, Components, Delegate> CreateClientMessageBuilder<Chain, Counterparty>
    for DelegateBuildCreateClientMessage<Components>
where
    Chain: HasMessageType + HasErrorType,
    Counterparty: HasCreateClientPayloadType<Chain>,
    Delegate: CreateClientMessageBuilder<Chain, Counterparty>,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
{
    async fn build_create_client_message(
        chain: &Chain,
        counterparty_payload: Counterparty::CreateClientPayload,
    ) -> Result<Chain::Message, Chain::Error> {
        Delegate::build_create_client_message(chain, counterparty_payload).await
    }
}
