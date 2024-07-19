use cgp_core::prelude::*;

#[derive_component(OutputTypeComponent, ProvideOutputType<App>)]
pub trait HasOutputType: Async {
    type Output: Async;
}

pub trait CanShowOutput<Value>: HasOutputType {
    fn show_output(&self, value: Value) -> Self::Output;
}
