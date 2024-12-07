use core::marker::PhantomData;

use cgp::core::component::UseContext;
use cgp::prelude::*;

#[cgp_component {
  name: RandomIdFlagGetterComponent,
  provider: RandomIdFlagGetter,
  context: Bootstrap,
}]
pub trait HasRandomIdFlag: Async {
    fn should_randomize_identifiers(&self) -> bool;
}

pub struct ReturnRandomIdFlag<const FLAG: bool>;

impl<Bootstrap, const FLAG: bool> RandomIdFlagGetter<Bootstrap> for ReturnRandomIdFlag<FLAG>
where
    Bootstrap: Async,
{
    fn should_randomize_identifiers(_bootstrap: &Bootstrap) -> bool {
        FLAG
    }
}

impl<Bootstrap> RandomIdFlagGetter<Bootstrap> for UseContext
where
    Bootstrap: Async + HasField<symbol!("should_randomize_identifiers"), Value = bool>,
{
    fn should_randomize_identifiers(bootstrap: &Bootstrap) -> bool {
        *bootstrap.get_field(PhantomData)
    }
}
