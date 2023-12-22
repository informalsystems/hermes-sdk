use cgp_core::prelude::*;
use cgp_core::CanRaiseError;

#[derive_component(RuntimeTypeComponent, ProvideRuntimeType<Context>)]
pub trait HasRuntimeType: Async {
    type Runtime: HasErrorType;
}

#[derive_component(RuntimeComponent, ProvideRuntime<Context>)]
pub trait HasRuntime:
    HasRuntimeType + CanRaiseError<<Self::Runtime as HasErrorType>::Error>
{
    fn runtime(&self) -> &Self::Runtime;
}
