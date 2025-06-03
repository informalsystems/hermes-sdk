use cgp::core::component::UseDelegate;
use hermes_chain_type_components::traits::{HasClientIdType, HasMessageType};
use hermes_prelude::*;

#[cgp_component {
  name: RecoverClientPayloadTypeComponent,
  provider: ProvideRecoverClientPayloadType,
  context: Chain,
}]
pub trait HasRecoverClientPayloadType: Async {
    type RecoverClientPayload: Async;
}

#[cgp_provider(RecoverClientPayloadTypeComponent)]
impl<Chain, Components, Delegate> ProvideRecoverClientPayloadType<Chain> for UseDelegate<Components>
where
    Chain: Async,
    Components: DelegateComponent<Chain, Delegate = Delegate>,
    Delegate: ProvideRecoverClientPayloadType<Chain>,
{
    type RecoverClientPayload = Delegate::RecoverClientPayload;
}

#[cgp_provider(RecoverClientPayloadTypeComponent)]
impl<Chain, Provider, RecoverClientPayload> ProvideRecoverClientPayloadType<Chain>
    for WithProvider<Provider>
where
    Chain: Async,
    RecoverClientPayload: Async,
    Provider: ProvideType<Chain, RecoverClientPayloadTypeComponent, Type = RecoverClientPayload>,
{
    type RecoverClientPayload = RecoverClientPayload;
}

#[cgp_component {
    provider: ClientRecovery,
    context: Chain,
}]
#[async_trait]
pub trait CanRecoverClient<Counterparty>:
    HasClientIdType<Counterparty> + HasRecoverClientPayloadType + HasMessageType
{
    /**
       Build message to recover client.
    */
    async fn recover_client_message(
        &self,
        subject_client: &Self::ClientId,
        substitute_client: &Self::ClientId,
        recover_client_payload: &Self::RecoverClientPayload,
    ) -> Self::Message;
}
