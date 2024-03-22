use cgp_core::prelude::*;
use hermes_runtime_components::traits::fs::file_path::{FilePathOf, HasFilePathType};
use hermes_runtime_components::traits::runtime::HasRuntimeType;

#[derive_component(RollupStoreDirGetterComponent, RollupStoreDirGetter<Bootstrap>)]
pub trait HasRollupStoreDir: HasRuntimeType
where
    Self::Runtime: HasFilePathType,
{
    fn rollup_store_dir(&self) -> &FilePathOf<Self::Runtime>;
}
