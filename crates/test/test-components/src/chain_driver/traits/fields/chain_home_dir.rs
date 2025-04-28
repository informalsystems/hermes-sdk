use hermes_prelude::*;
use hermes_runtime_components::traits::{FilePathOf, HasFilePathType, HasRuntime};

#[cgp_component {
  provider: ChainHomeDirGetter,
  context: ChainDriver,
}]
pub trait HasChainHomeDir: HasRuntime
where
    Self::Runtime: HasFilePathType,
{
    fn chain_home_dir(&self) -> &FilePathOf<Self::Runtime>;
}
