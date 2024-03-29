use cgp_core::prelude::*;

use crate::chain::traits::types::client_state::HasClientStateType;
use crate::chain::traits::types::height::HasHeightType;
use crate::chain::traits::types::update_client::HasUpdateClientPayloadType;

#[derive_component(UpdateClientPayloadBuilderComponent, UpdateClientPayloadBuilder<Chain>)]
#[async_trait]
pub trait CanBuildUpdateClientPayload<Counterparty>:
    HasUpdateClientPayloadType<Counterparty>
    + HasClientStateType<Counterparty>
    + HasHeightType
    + HasErrorType
{
    async fn build_update_client_payload(
        &self,
        trusted_height: &Self::Height,
        target_height: &Self::Height,
        client_state: Self::ClientState,
    ) -> Result<Self::UpdateClientPayload, Self::Error>;
}
