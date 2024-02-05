use hermes_relayer_components::runtime::traits::runtime::HasRuntimeType;
use hermes_test_components::runtime::traits::types::file_path::{FilePathOf, HasFilePathType};

pub trait HasRollupStoreDir: HasRuntimeType
where
    Self::Runtime: HasFilePathType,
{
    fn rollup_store_dir(&self) -> &FilePathOf<Self::Runtime>;
}
