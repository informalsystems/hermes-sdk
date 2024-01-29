use cgp_core::prelude::*;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_test_components::runtime::traits::types::file_path::{FilePathOf, HasFilePathType};

#[derive_component(ChainCommandPathComponent, ChainCommandPathGetter<Bootstrap>)]
pub trait HasChainCommandPath: HasRuntime
where
    Self::Runtime: HasFilePathType,
{
    fn chain_command_path(&self) -> &FilePathOf<Self::Runtime>;
}
