use cgp::core::component::UseDelegate;
use cgp::prelude::*;

use crate::traits::types::create_client::{
    HasCreateClientPayloadOptionsType, HasCreateClientPayloadType,
};

#[cgp_component {
  name: CreateClientPayloadBuilderComponent,
  provider: CreateClientPayloadBuilder,
  context: Chain,
}]
#[async_trait]
pub trait CanBuildCreateClientPayload<Counterparty>:
    HasCreateClientPayloadOptionsType<Counterparty>
    + HasCreateClientPayloadType<Counterparty>
    + HasErrorType
{
    async fn build_create_client_payload(
        &self,
        create_client_options: &Self::CreateClientPayloadOptions,
    ) -> Result<Self::CreateClientPayload, Self::Error>;
}

impl<Chain, Counterparty, Components, Delegate> CreateClientPayloadBuilder<Chain, Counterparty>
    for UseDelegate<Components>
where
    Chain: HasCreateClientPayloadOptionsType<Counterparty>
        + HasCreateClientPayloadType<Counterparty>
        + HasErrorType,
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
