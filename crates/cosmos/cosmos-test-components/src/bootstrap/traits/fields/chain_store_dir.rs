use core::marker::PhantomData;

use cgp::core::component::UseContext;
use cgp::prelude::*;
use hermes_runtime_components::traits::fs::file_path::{FilePathOf, HasFilePathType};
use hermes_runtime_components::traits::runtime::HasRuntime;

#[derive_component(ChainStoreDirGetterComponent, ChainStoreDirGetter<Bootstrap>)]
pub trait HasChainStoreDir: HasRuntime<Runtime: HasFilePathType> {
    fn chain_store_dir(&self) -> &FilePathOf<Self::Runtime>;
}

impl<Bootstrap, Runtime> ChainStoreDirGetter<Bootstrap> for UseContext
where
    Bootstrap: HasRuntime<Runtime = Runtime>
        + HasField<symbol!("chain_store_dir"), Field = Runtime::FilePath>,
    Runtime: HasFilePathType,
{
    fn chain_store_dir(bootstrap: &Bootstrap) -> &Runtime::FilePath {
        bootstrap.get_field(PhantomData)
    }
}
