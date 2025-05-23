use hermes_core::chain_type_components::traits::{HasAddressType, HasAmountType};
use hermes_core::test_components::chain::traits::HasProposalIdType;
use hermes_prelude::*;

#[cgp_component {
  provider: WasmClientCodeUploader,
  context: Chain,
}]
#[async_trait]
pub trait CanUploadWasmClientCode:
    HasAmountType + HasAddressType + HasProposalIdType + HasAsyncErrorType
{
    async fn upload_wasm_client_code(
        &self,
        wasm_client_bytes: &Vec<u8>,
        title: &str,
        summary: &str,
        authority: &Self::Address,
        deposit_amount: &Self::Amount,
    ) -> Result<Self::ProposalId, Self::Error>;
}
