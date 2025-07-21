use cgp::core::component::UseDelegate;
use hermes_chain_type_components::traits::HasHeightType;
use hermes_prelude::*;

use crate::traits::{
    ClientUpgradePayloadBuilder, ClientUpgradePayloadBuilderComponent, HasUpgradeClientPayloadType,
};

#[cgp_provider(ClientUpgradePayloadBuilderComponent)]
impl<Chain, Counterparty, Components, Delegate> ClientUpgradePayloadBuilder<Chain, Counterparty>
    for UseDelegate<Components>
where
    Chain: HasHeightType + HasUpgradeClientPayloadType + HasAsyncErrorType,
    Delegate: ClientUpgradePayloadBuilder<Chain, Counterparty>,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
{
    async fn upgrade_client_payload(
        chain: &Chain,
        upgrade_height: &Chain::Height,
    ) -> Result<Chain::UpgradeClientPayload, Chain::Error> {
        Delegate::upgrade_client_payload(chain, upgrade_height).await
    }
}
