use hermes_core::chain_components::traits::HasAddressType;
use hermes_cosmos_chain_components::types::HasWasmAccessType;
use hermes_prelude::*;

#[cgp_component {
  provider: WasmContractUploader,
  context: Chain,
}]
#[async_trait]
pub trait CanUploadWasmContract: HasAddressType + HasWasmAccessType + HasAsyncErrorType {
    async fn upload_wasm_contract(
        &self,
        wasm_client_bytes: &[u8],
        sender: &Self::Address,
        access_type: &Self::WasmAccess,
    ) -> Result<u64, Self::Error>;
}
