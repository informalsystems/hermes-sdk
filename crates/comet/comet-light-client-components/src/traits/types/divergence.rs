use cgp::core::component::WithProvider;
use cgp::core::types::ProvideType;
use hermes_prelude::*;

#[cgp_component {
  name: DivergenceTypeComponent,
  provider: ProvideDivergenceType,
  context: Client,
}]
pub trait HasDivergenceType: Async {
    type Divergence: Async;
}

#[cgp_provider(DivergenceTypeComponent)]
impl<Client: Async, Provider, Divergence> ProvideDivergenceType<Client> for WithProvider<Provider>
where
    Provider: ProvideType<Client, DivergenceTypeComponent, Type = Divergence>,
    Divergence: Async,
{
    type Divergence = Divergence;
}
