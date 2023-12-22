use cgp_core::prelude::*;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_test_components::runtime::traits::types::file_path::{FilePath, HasFilePathType};

#[derive_component(TestDirComponent, TestDirGetter<Bootstrap>)]
pub trait HasTestDir: HasRuntime
where
    Self::Runtime: HasFilePathType,
{
    fn test_dir(&self) -> &FilePath<Self::Runtime>;
}
