use core::marker::PhantomData;

use cgp::core::component::WithProvider;
use cgp::core::field::FieldGetter;
use cgp::core::types::traits::ProvideType;
use cgp::prelude::*;

#[derive_component(RuntimeTypeComponent, ProvideRuntimeType<Context>)]
pub trait HasRuntimeType: Async {
    type Runtime: Async + HasErrorType;
}

#[derive_component(RuntimeGetterComponent, RuntimeGetter<Context>)]
pub trait HasRuntime: HasRuntimeType {
    fn runtime(&self) -> &Self::Runtime;
}

pub type RuntimeOf<Context> = <Context as HasRuntimeType>::Runtime;

impl<Context, Provider, Runtime> ProvideRuntimeType<Context> for WithProvider<Provider>
where
    Context: Async,
    Provider: ProvideType<Context, RuntimeTypeComponent, Type = Runtime>,
    Runtime: Async + HasErrorType,
{
    type Runtime = Runtime;
}

impl<Context, Provider, Runtime> RuntimeGetter<Context> for WithProvider<Provider>
where
    Context: HasRuntimeType<Runtime = Runtime>,
    Provider: FieldGetter<Context, RuntimeGetterComponent, Field = Runtime>,
{
    fn runtime(context: &Context) -> &Runtime {
        Provider::get_field(context, PhantomData)
    }
}
