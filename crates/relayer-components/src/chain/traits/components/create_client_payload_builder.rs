use alloc::boxed::Box;

use cgp_core::prelude::*;

use crate::chain::traits::types::create_client::{HasCreateClientOptions, HasCreateClientPayload};

#[derive_component(CreateClientPayloadBuilderComponent, CreateClientPayloadBuilder<Chain>)]
#[async_trait]
pub trait CanBuildCreateClientPayload<Counterparty>:
    HasCreateClientOptions<Counterparty> + HasCreateClientPayload<Counterparty> + HasErrorType
{
    async fn build_create_client_payload(
        &self,
        create_client_options: &Self::CreateClientPayloadOptions,
    ) -> Result<Self::CreateClientPayload, Self::Error>;
}
