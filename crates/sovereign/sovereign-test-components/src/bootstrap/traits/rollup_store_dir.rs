use cgp_core::prelude::*;
use hermes_relayer_components::runtime::traits::runtime::HasRuntimeType;
use hermes_test_components::runtime::traits::types::file_path::{FilePathOf, HasFilePathType};

#[derive_component(RollupStoreDirGetterComponent, RollupStoreDirGetter<Bootstrap>)]
pub trait HasRollupStoreDir: HasRuntimeType
where
    Self::Runtime: HasFilePathType,
{
    fn rollup_store_dir(&self) -> &FilePathOf<Self::Runtime>;
}
