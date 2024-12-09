use cgp::prelude::*;
use hermes_test_components::chain::traits::proposal::types::proposal_id::HasProposalIdType;
use hermes_test_components::chain::traits::types::address::HasAddressType;
use hermes_test_components::chain::traits::types::amount::HasAmountType;

#[cgp_component {
  provider: WasmClientCodeUploader,
  context: Chain,
}]
#[async_trait]
pub trait CanUploadWasmClientCode:
    HasAmountType + HasAddressType + HasProposalIdType + HasErrorType
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
