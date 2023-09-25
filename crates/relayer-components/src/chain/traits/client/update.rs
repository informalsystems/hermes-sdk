use async_trait::async_trait;

use crate::chain::traits::types::client_state::HasClientStateType;
use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::std_prelude::*;
use cgp_core::traits::error::HasErrorType;
use cgp_core::traits::sync::Async;

pub trait HasUpdateClientPayload<Counterparty>: HasIbcChainTypes<Counterparty> {
    type UpdateClientPayload: Async;
}

#[async_trait]
pub trait CanBuildUpdateClientPayload<Counterparty>:
    HasUpdateClientPayload<Counterparty> + HasClientStateType<Counterparty> + HasErrorType
{
    async fn build_update_client_payload(
        &self,
        trusted_height: &Self::Height,
        target_height: &Self::Height,
        client_state: Self::ClientState,
    ) -> Result<Self::UpdateClientPayload, Self::Error>;
}

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
