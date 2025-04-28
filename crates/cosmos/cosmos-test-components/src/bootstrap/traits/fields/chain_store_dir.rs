use hermes_core::runtime_components::traits::{FilePathOf, HasFilePathType, HasRuntime};
use hermes_prelude::*;

#[cgp_getter {
    provider: ChainStoreDirGetter,
}]
pub trait HasChainStoreDir: HasRuntime<Runtime: HasFilePathType> {
    fn chain_store_dir(&self) -> &FilePathOf<Self::Runtime>;
}
