use cgp_core::prelude::*;
use hermes_runtime_components::traits::runtime::HasRuntime;
use hermes_test_components::runtime::traits::types::file_path::{FilePathOf, HasFilePathType};

#[derive_component(BridgeStoreDirGetterComponent, BridgeStoreDirGetter<Bootstrap>)]
pub trait HasBridgeStoreDir: HasRuntime
where
    Self::Runtime: HasFilePathType,
{
    fn bridge_store_dir(&self) -> &FilePathOf<Self::Runtime>;
}
