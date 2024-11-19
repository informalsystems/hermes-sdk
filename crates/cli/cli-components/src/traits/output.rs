use cgp::core::component::WithProvider;
use cgp::core::types::traits::ProvideType;
use cgp::prelude::*;

#[derive_component(OutputTypeComponent, ProvideOutputType<App>)]
pub trait HasOutputType: Async {
    type Output: Async;
}

#[derive_component(OutputProducerComponent, OutputProducer<App>)]
pub trait CanProduceOutput<Value>: HasOutputType {
    fn produce_output(&self, value: Value) -> Self::Output;
}

impl<App, Provider, Output> ProvideOutputType<App> for WithProvider<Provider>
where
    App: Async,
    Output: Async,
    Provider: ProvideType<OutputTypeComponent, App, Type = Output>,
{
    type Output = Output;
}
