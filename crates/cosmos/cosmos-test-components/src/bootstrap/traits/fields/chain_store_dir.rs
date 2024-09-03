use cgp::prelude::*;
use hermes_runtime_components::traits::fs::file_path::{FilePathOf, HasFilePathType};
use hermes_runtime_components::traits::runtime::HasRuntime;

#[derive_component(ChainStoreDirGetterComponent, ChainStoreDirGetter<Bootstrap>)]
pub trait HasChainStoreDir: HasRuntime
where
    Self::Runtime: HasFilePathType,
{
    fn chain_store_dir(&self) -> &FilePathOf<Self::Runtime>;
}
