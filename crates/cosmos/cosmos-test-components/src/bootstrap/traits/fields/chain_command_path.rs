use core::marker::PhantomData;

use cgp::core::component::UseContext;
use cgp::prelude::*;
use hermes_runtime_components::traits::fs::file_path::{FilePathOf, HasFilePathType};
use hermes_runtime_components::traits::runtime::HasRuntime;

#[cgp_component {
  provider: ChainCommandPathGetter,
  context: Bootstrap,
}]
pub trait HasChainCommandPath: HasRuntime
where
    Self::Runtime: HasFilePathType,
{
    fn chain_command_path(&self) -> &FilePathOf<Self::Runtime>;
}

#[cgp_provider(ChainCommandPathGetterComponent)]
impl<Bootstrap, Runtime> ChainCommandPathGetter<Bootstrap> for UseContext
where
    Bootstrap: HasRuntime<Runtime = Runtime>
        + HasField<symbol!("chain_command_path"), Value = Runtime::FilePath>,
    Runtime: HasFilePathType,
{
    fn chain_command_path(bootstrap: &Bootstrap) -> &Runtime::FilePath {
        bootstrap.get_field(PhantomData)
    }
}
