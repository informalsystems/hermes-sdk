use hermes_prelude::*;

#[cgp_getter {
    provider: RandomIdFlagGetter,
}]
pub trait HasRandomIdFlag {
    fn should_randomize_identifiers(&self) -> bool;
}

pub struct UseRandomIdFlag<const FLAG: bool>;

#[cgp_provider(RandomIdFlagGetterComponent)]
impl<Bootstrap, const FLAG: bool> RandomIdFlagGetter<Bootstrap> for UseRandomIdFlag<FLAG>
where
    Bootstrap: Async,
{
    fn should_randomize_identifiers(_bootstrap: &Bootstrap) -> bool {
        FLAG
    }
}
