use cgp::core::component::UseDelegate;
use hermes_prelude::*;

use crate::traits::{HasCreateClientPayloadOptionsType, HasCreateClientPayloadType};

#[cgp_component {
  provider: CreateClientPayloadBuilder,
  context: Chain,
}]
#[async_trait]
pub trait CanBuildCreateClientPayload<Counterparty>:
    HasCreateClientPayloadOptionsType<Counterparty>
    + HasCreateClientPayloadType<Counterparty>
    + HasAsyncErrorType
{
    async fn build_create_client_payload(
        &self,
        create_client_options: &Self::CreateClientPayloadOptions,
    ) -> Result<Self::CreateClientPayload, Self::Error>;
}

#[cgp_provider(CreateClientPayloadBuilderComponent)]
impl<Chain, Counterparty, Components, Delegate> CreateClientPayloadBuilder<Chain, Counterparty>
    for UseDelegate<Components>
where
    Chain: HasCreateClientPayloadOptionsType<Counterparty>
        + HasCreateClientPayloadType<Counterparty>
        + HasAsyncErrorType,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
    Delegate: CreateClientPayloadBuilder<Chain, Counterparty>,
{
    async fn build_create_client_payload(
        chain: &Chain,
        create_client_options: &Chain::CreateClientPayloadOptions,
    ) -> Result<Chain::CreateClientPayload, Chain::Error> {
        Delegate::build_create_client_payload(chain, create_client_options).await
    }
}
