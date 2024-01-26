use cgp_core::prelude::*;

use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_test_components::runtime::traits::types::file_path::{FilePath, HasFilePathType};

#[derive_component(BridgeStoreDirGetterComponent, BridgeStoreDirGetter<Bootstrap>)]
pub trait HasBridgeStoreDir: HasRuntime
where
    Self::Runtime: HasFilePathType,
{
    fn bridge_store_dir(&self) -> &FilePath<Self::Runtime>;
}
