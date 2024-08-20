use cgp_core::prelude::*;
use hermes_test_components::chain::traits::types::amount::HasAmountType;

#[derive_component(WasmClientCodeUploaderComponent, WasmClientCodeUploader<Chain>)]
#[async_trait]
pub trait CanUploadWasmClientCode: HasAmountType + HasErrorType {
    async fn upload_wasm_client_code(
        &self,
        wasm_client_bytes: &Vec<u8>,
        title: &str,
        summary: &str,
        deposit_amount: &Self::Amount,
    ) -> Result<(), Self::Error>;
}
