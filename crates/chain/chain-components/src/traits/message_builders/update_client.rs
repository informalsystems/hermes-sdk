use alloc::vec::Vec;

use cgp::core::component::UseDelegate;
use cgp::prelude::*;
use hermes_chain_type_components::traits::types::counterparty::CanUseCounterparty;
use hermes_chain_type_components::traits::types::ibc::client_id::HasClientIdType;
use hermes_chain_type_components::traits::types::message::HasMessageType;

use crate::traits::types::update_client::{HasUpdateClientPayloadType, UpdateClientPayloadOf};

#[cgp_component {
  provider: UpdateClientMessageBuilder,
  context: Chain,
}]
#[async_trait]
pub trait CanBuildUpdateClientMessage<Counterparty>:
    HasClientIdType<Counterparty>
    + CanUseCounterparty<Counterparty, Counterparty: HasUpdateClientPayloadType<Self>>
    + HasMessageType
    + HasErrorType
{
    async fn build_update_client_message(
        &self,
        client_id: &Self::ClientId,
        payload: UpdateClientPayloadOf<Counterparty, Self>,
    ) -> Result<Vec<Self::Message>, Self::Error>;
}

impl<Chain, Counterparty, Components, Delegate> UpdateClientMessageBuilder<Chain, Counterparty>
    for UseDelegate<Components>
where
    Chain: HasClientIdType<Counterparty> + HasMessageType + HasErrorType,
    Counterparty: HasUpdateClientPayloadType<Chain>,
    Delegate: UpdateClientMessageBuilder<Chain, Counterparty>,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
{
    async fn build_update_client_message(
        chain: &Chain,
        client_id: &Chain::ClientId,
        payload: Counterparty::UpdateClientPayload,
    ) -> Result<Vec<Chain::Message>, Chain::Error> {
        Delegate::build_update_client_message(chain, client_id, payload).await
    }
}
