use core::marker::PhantomData;

use cgp::core::component::WithProvider;
use cgp::core::field::FieldGetter;
use cgp::core::types::ProvideType;
use cgp::prelude::*;

#[cgp_component {
  name: RuntimeTypeComponent,
  provider: ProvideRuntimeType,
}]
pub trait HasRuntimeType: Async {
    type Runtime: Async + HasAsyncErrorType;
}

#[cgp_component {
  provider: RuntimeGetter,
}]
pub trait HasRuntime: HasRuntimeType {
    fn runtime(&self) -> &Self::Runtime;
}

pub type RuntimeOf<Context> = <Context as HasRuntimeType>::Runtime;

impl<Context, Provider, Runtime> ProvideRuntimeType<Context> for WithProvider<Provider>
where
    Context: Async,
    Provider: ProvideType<Context, RuntimeTypeComponent, Type = Runtime>,
    Runtime: Async + HasAsyncErrorType,
{
    type Runtime = Runtime;
}

impl<Context, Provider, Runtime> RuntimeGetter<Context> for WithProvider<Provider>
where
    Context: HasRuntimeType<Runtime = Runtime>,
    Provider: FieldGetter<Context, RuntimeGetterComponent, Value = Runtime>,
{
    fn runtime(context: &Context) -> &Runtime {
        Provider::get_field(context, PhantomData)
    }
}
