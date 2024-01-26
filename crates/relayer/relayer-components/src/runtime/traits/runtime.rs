use cgp_core::prelude::*;

#[derive_component(RuntimeTypeComponent, ProvideRuntimeType<Context>)]
pub trait HasRuntimeType: Async {
    type Runtime: HasErrorType;
}

#[derive_component(RuntimeComponent, ProvideRuntime<Context>)]
pub trait HasRuntime: HasRuntimeType {
    fn runtime(&self) -> &Self::Runtime;
}
