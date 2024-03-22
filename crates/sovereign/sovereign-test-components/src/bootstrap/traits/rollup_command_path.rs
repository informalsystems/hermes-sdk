use cgp_core::prelude::*;
use hermes_runtime_components::traits::fs::file_path::{FilePathOf, HasFilePathType};
use hermes_runtime_components::traits::runtime::HasRuntimeType;

#[derive_component(RollupCommandPathGetterComponent, RollupCommandPathGetter<Bootstrap>)]
pub trait HasRollupCommandPath: HasRuntimeType
where
    Self::Runtime: HasFilePathType,
{
    fn rollup_command_path(&self) -> &FilePathOf<Self::Runtime>;
}
