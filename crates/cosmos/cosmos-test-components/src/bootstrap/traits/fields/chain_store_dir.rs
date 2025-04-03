use cgp::prelude::*;
use hermes_runtime_components::traits::fs::file_path::{FilePathOf, HasFilePathType};
use hermes_runtime_components::traits::runtime::HasRuntime;

#[cgp_getter {
    provider: ChainStoreDirGetter,
}]
pub trait HasChainStoreDir: HasRuntime<Runtime: HasFilePathType> {
    fn chain_store_dir(&self) -> &FilePathOf<Self::Runtime>;
}
