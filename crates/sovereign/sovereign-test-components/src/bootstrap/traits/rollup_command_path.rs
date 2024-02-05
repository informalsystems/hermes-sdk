use cgp_core::prelude::*;
use hermes_relayer_components::runtime::traits::runtime::HasRuntimeType;
use hermes_test_components::runtime::traits::types::file_path::{FilePathOf, HasFilePathType};

#[derive_component(RollupCommandPathGetterComponent, RollupCommandPathGetter<Bootstrap>)]
pub trait HasRollupCommandPath: HasRuntimeType
where
    Self::Runtime: HasFilePathType,
{
    fn rollup_command_path(&self) -> &FilePathOf<Self::Runtime>;
}
