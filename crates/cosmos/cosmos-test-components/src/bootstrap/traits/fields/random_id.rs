use core::marker::PhantomData;

use cgp::core::component::UseContext;
use cgp::prelude::*;

#[derive_component(RandomIdFlagGetterComponent, RandomIdFlagGetter<Bootstrap>)]
pub trait HasRandomIdFlag: Async {
    fn should_randomize_identifiers(&self) -> bool;
}

impl<Bootstrap> RandomIdFlagGetter<Bootstrap> for UseContext
where
    Bootstrap: Async + HasField<symbol!("should_randomize_identifiers"), Field = bool>,
{
    fn should_randomize_identifiers(bootstrap: &Bootstrap) -> bool {
        *bootstrap.get_field(PhantomData)
    }
}
