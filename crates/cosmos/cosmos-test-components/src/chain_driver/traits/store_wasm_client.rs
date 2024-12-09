use cgp::prelude::*;
use hermes_runtime_components::traits::fs::file_path::{FilePathOf, HasFilePathType};
use hermes_runtime_components::traits::runtime::HasRuntimeType;

#[cgp_component {
  provider: WasmClientCodeUploader,
  context: ChainDriver,
}]
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
