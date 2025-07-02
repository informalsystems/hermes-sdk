use hermes_chain_type_components::traits::HasAddressType;
use hermes_prelude::*;

#[cgp_component {
  provider: WasmContractUploader,
  context: Chain,
}]
#[async_trait]
pub trait CanUploadWasmContract: HasAddressType + HasAsyncErrorType {
    async fn upload_wasm_contract(
        &self,
        wasm_client_bytes: &[u8],
        sender: &Self::Address,
    ) -> Result<(), Self::Error>;
}
