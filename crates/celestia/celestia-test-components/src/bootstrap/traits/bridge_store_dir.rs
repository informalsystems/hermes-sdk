use cgp::prelude::*;
use hermes_runtime_components::traits::fs::file_path::{FilePathOf, HasFilePathType};
use hermes_runtime_components::traits::runtime::HasRuntime;

#[cgp_component {
  name: BridgeStoreDirGetterComponent,
  provider: BridgeStoreDirGetter,
  context: Bootstrap,
}]
pub trait HasBridgeStoreDir: HasRuntime
where
    Self::Runtime: HasFilePathType,
{
    fn bridge_store_dir(&self) -> &FilePathOf<Self::Runtime>;
}
