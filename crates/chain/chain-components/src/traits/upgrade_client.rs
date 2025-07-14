use cgp::core::component::UseDelegate;
use hermes_chain_type_components::traits::{HasClientIdType, HasHeightType, HasMessageType};
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
    HasClientIdType<Counterparty> + HasMessageType + HasAsyncErrorType
where
    Counterparty: HasUpgradeClientPayloadType,
{
    /**
       Build message to upgrade client.
    */
    async fn upgrade_client_message(
        &self,
        client_id: &Self::ClientId,
        upgrade_client_payload: &Counterparty::UpgradeClientPayload,
    ) -> Result<Self::Message, Self::Error>;
}

#[cgp_component {
    provider: ClientUpgradePayloadBuilder,
    context: Chain,
}]
#[async_trait]
pub trait CanBuildClientUpgradePayload<Counterparty>:
    HasHeightType + HasUpgradeClientPayloadType + HasAsyncErrorType
{
    /**
       Build message to upgrade client.
    */
    async fn upgrade_client_payload(
        &self,
        upgrade_height: &Self::Height,
    ) -> Result<Self::UpgradeClientPayload, Self::Error>;
}
