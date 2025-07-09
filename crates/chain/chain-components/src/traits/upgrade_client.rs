use cgp::core::component::UseDelegate;
use hermes_chain_type_components::traits::{HasClientIdType, HasMessageType};
use hermes_prelude::*;

#[cgp_component {
  name: UpgradeClientPayloadTypeComponent,
  provider: ProvideUpgradeClientPayloadType,
  context: Chain,
}]
pub trait HasUpgradeClientPayloadType: Async {
    type UpgradeClientPayload: Async;
}

#[cgp_provider(UpgradeClientPayloadTypeComponent)]
impl<Chain, Components, Delegate> ProvideUpgradeClientPayloadType<Chain> for UseDelegate<Components>
where
    Chain: Async,
    Components: DelegateComponent<Chain, Delegate = Delegate>,
    Delegate: ProvideUpgradeClientPayloadType<Chain>,
{
    type UpgradeClientPayload = Delegate::UpgradeClientPayload;
}

#[cgp_provider(UpgradeClientPayloadTypeComponent)]
impl<Chain, Provider, UpgradeClientPayload> ProvideUpgradeClientPayloadType<Chain>
    for WithProvider<Provider>
where
    Chain: Async,
    UpgradeClientPayload: Async,
    Provider: ProvideType<Chain, UpgradeClientPayloadTypeComponent, Type = UpgradeClientPayload>,
{
    type UpgradeClientPayload = UpgradeClientPayload;
}

#[cgp_component {
    provider: ClientUpgrade,
    context: Chain,
}]
#[async_trait]
pub trait CanUpgradeClient<Counterparty>:
    HasClientIdType<Counterparty> + HasUpgradeClientPayloadType + HasMessageType
{
    /**
       Build message to upgrade client.
    */
    async fn upgrade_client_message(
        &self,
        client_id: &Self::ClientId,
        upgrade_client_payload: &Self::UpgradeClientPayload,
    ) -> Self::Message;
}
