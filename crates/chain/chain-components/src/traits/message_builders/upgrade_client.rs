use cgp::core::component::UseDelegate;
use hermes_chain_type_components::traits::{HasClientIdType, HasMessageType};
use hermes_prelude::*;

use crate::traits::{ClientUpgrade, ClientUpgradeComponent, HasUpgradeClientPayloadType};

#[cgp_provider(ClientUpgradeComponent)]
impl<Chain, Counterparty, Components, Delegate> ClientUpgrade<Chain, Counterparty>
    for UseDelegate<Components>
where
    Chain: HasClientIdType<Counterparty> + HasMessageType + HasAsyncErrorType,
    Counterparty: HasUpgradeClientPayloadType,
    Delegate: ClientUpgrade<Chain, Counterparty>,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
{
    async fn upgrade_client_message(
        chain: &Chain,
        client_id: &Chain::ClientId,
        upgrade_client_payload: &Counterparty::UpgradeClientPayload,
    ) -> Result<Chain::Message, Chain::Error> {
        Delegate::upgrade_client_message(chain, client_id, upgrade_client_payload).await
    }
}
