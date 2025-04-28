use hermes_prelude::*;
use hermes_runtime_components::traits::{FilePathOf, HasFilePathType, HasRuntime};

#[cgp_component {
  provider: BridgeStoreDirGetter,
  context: Bootstrap,
}]
pub trait HasBridgeStoreDir: HasRuntime
where
    Self::Runtime: HasFilePathType,
{
    fn bridge_store_dir(&self) -> &FilePathOf<Self::Runtime>;
}
