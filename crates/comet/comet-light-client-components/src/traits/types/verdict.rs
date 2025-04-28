use cgp::core::component::WithProvider;
use cgp::core::types::ProvideType;
use hermes_prelude::*;

#[cgp_component {
  name: VerdictTypeComponent,
  provider: ProvideVerdictType,
  context: Client,
}]
pub trait HasVerdictType: Async {
    type Verdict: Async;
}

#[cgp_provider(VerdictTypeComponent)]
impl<Client: Async, Provider, Verdict> ProvideVerdictType<Client> for WithProvider<Provider>
where
    Provider: ProvideType<Client, VerdictTypeComponent, Type = Verdict>,
    Verdict: Async,
{
    type Verdict = Verdict;
}
