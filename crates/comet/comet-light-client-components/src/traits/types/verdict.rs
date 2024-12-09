use cgp::core::component::WithProvider;
use cgp::core::types::traits::ProvideType;
use cgp::prelude::*;

#[cgp_component {
  name: VerdictTypeComponent,
  provider: ProvideVerdictType,
  context: Client,
}]
pub trait HasVerdictType: Async {
    type Verdict: Async;
}

impl<Client: Async, Provider, Verdict> ProvideVerdictType<Client> for WithProvider<Provider>
where
    Provider: ProvideType<Client, VerdictTypeComponent, Type = Verdict>,
    Verdict: Async,
{
    type Verdict = Verdict;
}
