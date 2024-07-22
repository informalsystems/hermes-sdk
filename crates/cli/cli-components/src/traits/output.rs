use cgp_core::prelude::*;

#[derive_component(OutputTypeComponent, ProvideOutputType<App>)]
pub trait HasOutputType: Async {
    type Output: Async;
}

#[derive_component(OutputProducerComponent, OutputProducer<App>)]
pub trait CanProduceOutput<Value>: HasOutputType {
    fn produce_output(&self, value: Value) -> Self::Output;
}
