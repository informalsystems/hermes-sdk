use cgp_core::prelude::*;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use ibc_test_components::runtime::traits::types::file_path::{FilePath, HasFilePathType};

#[derive_component(ChainCommandPathComponent, ChainCommandPathGetter<Bootstrap>)]
pub trait HasChainCommandPath: HasRuntime
where
    Self::Runtime: HasFilePathType,
{
    fn chain_command_path(&self) -> &FilePath<Self::Runtime>;
}
