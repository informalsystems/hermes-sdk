use cgp_core::prelude::*;

use crate::chain::traits::types::create_client::{
    HasCreateClientOptionsType, HasCreateClientPayloadType,
};

#[derive_component(CreateClientPayloadBuilderComponent, CreateClientPayloadBuilder<Chain>)]
#[async_trait]
pub trait CanBuildCreateClientPayload<Counterparty>:
    HasCreateClientOptionsType<Counterparty> + HasCreateClientPayloadType<Counterparty> + HasErrorType
{
    async fn build_create_client_payload(
        &self,
        create_client_options: &Self::CreateClientOptions,
    ) -> Result<Self::CreateClientPayload, Self::Error>;
}
