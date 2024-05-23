use cgp_core::prelude::*;
use hermes_runtime_components::traits::fs::file_path::{FilePathOf, HasFilePathType};
use hermes_runtime_components::traits::runtime::HasRuntime;

#[derive_component(WasmClientCodePathGetterComponent, WasmClientCodePathGetter<Bootstrap>)]
pub trait HasWasmClientCodePath: HasRuntime
where
    Self::Runtime: HasFilePathType,
{
    fn wasm_client_code_path(&self) -> &FilePathOf<Self::Runtime>;
}
