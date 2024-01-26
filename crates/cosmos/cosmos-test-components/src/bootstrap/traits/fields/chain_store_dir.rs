use cgp_core::prelude::*;
use hermes_relayer_components::runtime::traits::runtime::HasRuntime;
use hermes_test_components::runtime::traits::types::file_path::{FilePath, HasFilePathType};

#[derive_component(ChainStoreDirGetterComponent, ChainStoreDirGetter<Bootstrap>)]
pub trait HasChainStoreDir: HasRuntime
where
    Self::Runtime: HasFilePathType,
{
    fn chain_store_dir(&self) -> &FilePath<Self::Runtime>;
}
