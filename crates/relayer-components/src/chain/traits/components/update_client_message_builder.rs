use alloc::boxed::Box;
use alloc::vec::Vec;
use cgp_core::prelude::*;

use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::chain::traits::types::update_client::HasUpdateClientPayload;

#[derive_component(UpdateClientMessageBuilderComponent, UpdateClientMessageBuilder<Chain>)]
#[async_trait]
pub trait CanBuildUpdateClientMessage<Counterparty>:
    HasIbcChainTypes<Counterparty> + HasErrorType
where
    Counterparty: HasUpdateClientPayload<Self>,
{
    async fn build_update_client_message(
        &self,
        client_id: &Self::ClientId,
        payload: Counterparty::UpdateClientPayload,
    ) -> Result<Vec<Self::Message>, Self::Error>;
}
