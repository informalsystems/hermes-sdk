use cgp::prelude::*;

#[derive_component(ConfigTypeComponent, ProvideConfigType<App>)]
pub trait HasConfigType: Async {
    type Config: Async;
}
