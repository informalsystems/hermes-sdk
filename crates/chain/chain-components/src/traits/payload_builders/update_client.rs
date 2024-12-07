use cgp::core::component::UseDelegate;
use cgp::prelude::*;

use crate::traits::types::client_state::HasClientStateType;
use crate::traits::types::height::HasHeightType;
use crate::traits::types::update_client::HasUpdateClientPayloadType;

#[cgp_component {
  name: UpdateClientPayloadBuilderComponent,
  provider: UpdateClientPayloadBuilder,
  context: Chain,
}]
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

impl<Chain, Counterparty, Components, Delegate> UpdateClientPayloadBuilder<Chain, Counterparty>
    for UseDelegate<Components>
where
    Chain: HasUpdateClientPayloadType<Counterparty>
        + HasClientStateType<Counterparty>
        + HasHeightType
        + HasErrorType,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
    Delegate: UpdateClientPayloadBuilder<Chain, Counterparty>,
{
    async fn build_update_client_payload(
        chain: &Chain,
        trusted_height: &Chain::Height,
        target_height: &Chain::Height,
        client_state: Chain::ClientState,
    ) -> Result<Chain::UpdateClientPayload, Chain::Error> {
        Delegate::build_update_client_payload(chain, trusted_height, target_height, client_state)
            .await
    }
}
