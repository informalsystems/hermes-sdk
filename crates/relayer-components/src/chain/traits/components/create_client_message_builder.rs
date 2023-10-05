use cgp_core::prelude::*;

use crate::chain::traits::types::create_client::HasCreateClientPayload;
use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::std_prelude::*;

#[derive_component(CreateClientMessageBuilderComponent, CreateClientMessageBuilder<Chain>)]
#[async_trait]
#[async_trait]
pub trait CanBuildCreateClientMessage<Counterparty>:
    HasIbcChainTypes<Counterparty> + HasErrorType
where
    Counterparty: HasCreateClientPayload<Self>,
{
    async fn build_create_client_message(
        &self,
        counterparty_payload: Counterparty::CreateClientPayload,
    ) -> Result<Self::Message, Self::Error>;
}
