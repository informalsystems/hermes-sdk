use cgp::core::component::WithProvider;
use cgp::core::types::ProvideType;
use hermes_prelude::*;

#[cgp_component {
  name: OutputTypeComponent,
  provider: ProvideOutputType,
  context: App,
}]
pub trait HasOutputType: Async {
    type Output: Async;
}

#[cgp_component {
  provider: OutputProducer,
  context: App,
}]
pub trait CanProduceOutput<Value>: HasOutputType {
    fn produce_output(&self, value: Value) -> Self::Output;
}

#[cgp_provider(OutputTypeComponent)]
impl<App, Provider, Output> ProvideOutputType<App> for WithProvider<Provider>
where
    App: Async,
    Output: Async,
    Provider: ProvideType<OutputTypeComponent, App, Type = Output>,
{
    type Output = Output;
}
