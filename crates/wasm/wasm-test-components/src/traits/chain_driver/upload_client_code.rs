use cgp_core::prelude::*;
use hermes_runtime_components::traits::fs::file_path::{FilePathOf, HasFilePathType};
use hermes_runtime_components::traits::runtime::HasRuntimeType;
use hermes_test_components::chain::traits::types::wallet::{HasWalletType, WalletOf};
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;

#[derive_component(WasmClientCodeUploaderComponent, WasmClientCodeUploader<ChainDriver>)]
#[async_trait]
pub trait CanUploadWasmClientCode: HasChainType + HasRuntimeType + HasErrorType
where
    Self::Runtime: HasFilePathType,
    Self::Chain: HasWalletType,
{
    async fn upload_wasm_client_code(
        &self,
        wasm_client_code_path: &FilePathOf<Self::Runtime>,
        title: &str,
        summary: &str,
        sender: &WalletOf<Self::Chain>,
    ) -> Result<String, Self::Error>;
}
