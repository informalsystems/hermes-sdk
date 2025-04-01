use cgp::prelude::*;
use hermes_runtime_components::traits::fs::file_path::{FilePathOf, HasFilePathType};
use hermes_runtime_components::traits::runtime::HasRuntime;

#[cgp_getter {
    provider: ChainCommandPathGetter,
}]
pub trait HasChainCommandPath: HasRuntime
where
    Self::Runtime: HasFilePathType,
{
    fn chain_command_path(&self) -> &FilePathOf<Self::Runtime>;
}
