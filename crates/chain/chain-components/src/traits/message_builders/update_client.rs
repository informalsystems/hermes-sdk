use alloc::vec::Vec;

use cgp::core::component::DelegateTo;
use cgp::prelude::*;

use crate::traits::types::ibc::HasIbcChainTypes;
use crate::traits::types::update_client::HasUpdateClientPayloadType;

#[derive_component(UpdateClientMessageBuilderComponent, UpdateClientMessageBuilder<Chain>)]
#[async_trait]
pub trait CanBuildUpdateClientMessage<Counterparty>:
    HasIbcChainTypes<Counterparty> + HasErrorType
where
    Counterparty: HasUpdateClientPayloadType<Self>,
{
    async fn build_update_client_message(
        &self,
        client_id: &Self::ClientId,
        payload: Counterparty::UpdateClientPayload,
    ) -> Result<Vec<Self::Message>, Self::Error>;
}

impl<Chain, Counterparty, Components, Delegate> UpdateClientMessageBuilder<Chain, Counterparty>
    for DelegateTo<Components>
where
    Chain: HasIbcChainTypes<Counterparty> + HasErrorType,
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
