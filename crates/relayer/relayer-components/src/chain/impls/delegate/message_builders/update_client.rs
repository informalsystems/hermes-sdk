use alloc::vec::Vec;
use core::marker::PhantomData;

use cgp_core::{DelegateComponent, HasErrorType};

use crate::chain::traits::message_builders::update_client::UpdateClientMessageBuilder;
use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::chain::traits::types::update_client::HasUpdateClientPayloadType;

pub struct DelegateBuildUpdateClientMessage<Components>(pub PhantomData<Components>);

impl<Chain, Counterparty, Components, Delegate> UpdateClientMessageBuilder<Chain, Counterparty>
    for DelegateBuildUpdateClientMessage<Components>
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
