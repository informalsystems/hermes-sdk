use cgp_core::prelude::*;

#[derive_component(RandomIdFlagComponent, RandomIdFlagGetter<Bootstrap>)]
pub trait HasRandomIdFlag: Async {
    fn should_randomize_identifiers(&self) -> bool;
}
