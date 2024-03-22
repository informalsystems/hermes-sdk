use cgp_core::prelude::*;
use hermes_runtime_components::traits::fs::file_path::{FilePathOf, HasFilePathType};
use hermes_runtime_components::traits::runtime::HasRuntime;

#[derive_component(ChainHomeDirGetterComponent, ChainHomeDirGetter<ChainDriver>)]
pub trait HasChainHomeDir: HasRuntime
where
    Self::Runtime: HasFilePathType,
{
    fn chain_home_dir(&self) -> &FilePathOf<Self::Runtime>;
}
