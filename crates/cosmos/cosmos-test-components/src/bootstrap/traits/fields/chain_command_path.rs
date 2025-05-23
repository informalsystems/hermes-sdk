use hermes_core::runtime_components::traits::{FilePathOf, HasFilePathType, HasRuntime};
use hermes_prelude::*;

#[cgp_getter {
    provider: ChainCommandPathGetter,
}]
pub trait HasChainCommandPath: HasRuntime
where
    Self::Runtime: HasFilePathType,
{
    fn chain_command_path(&self) -> &FilePathOf<Self::Runtime>;
}
