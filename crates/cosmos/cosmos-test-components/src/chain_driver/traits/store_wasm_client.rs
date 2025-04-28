use hermes_prelude::*;
use hermes_core::runtime_components::traits::{FilePathOf, HasFilePathType};
use hermes_core::runtime_components::traits::HasRuntimeType;

#[cgp_component {
  provider: WasmClientCodeUploader,
  context: ChainDriver,
}]
#[async_trait]
pub trait CanUploadWasmClientCode: HasRuntimeType + HasAsyncErrorType
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
