use cgp_core::prelude::*;
use hermes_relayer_components::runtime::traits::runtime::HasRuntimeType;
use hermes_test_components::runtime::traits::types::file_path::{FilePathOf, HasFilePathType};

#[derive_component(WasmClientCodeUploaderComponent, WasmClientCodeUploader<ChainDriver>)]
#[async_trait]
pub trait CanUploadWasmClientCode: HasRuntimeType + HasErrorType
where
    Self::Runtime: HasFilePathType,
{
    async fn store_wasm_client_code(
        &self,
        wasm_client_code_path: &FilePathOf<Self::Runtime>,
        title: &str,
        summary: &str,
        sender: &str,
    ) -> Result<String, Self::Error>;
}
