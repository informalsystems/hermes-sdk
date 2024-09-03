use cgp::prelude::*;

use crate::chain::traits::types::create_client::{
    HasCreateClientPayloadOptionsType, HasCreateClientPayloadType,
};

#[derive_component(CreateClientPayloadBuilderComponent, CreateClientPayloadBuilder<Chain>)]
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
