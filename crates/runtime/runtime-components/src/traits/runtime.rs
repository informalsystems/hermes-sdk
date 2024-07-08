use core::marker::PhantomData;

use cgp_core::prelude::*;

#[derive_component(RuntimeTypeComponent, ProvideRuntimeType<Context>)]
pub trait HasRuntimeType: Async {
    type Runtime: HasErrorType;
}

#[derive_component(RuntimeGetterComponent, RuntimeGetter<Context>)]
pub trait HasRuntime: HasRuntimeType {
    fn runtime(&self) -> &Self::Runtime;
}

pub type RuntimeOf<Context> = <Context as HasRuntimeType>::Runtime;

pub struct ProvideRuntimeField<Field>(pub PhantomData<Field>);

impl<Context, Field: Async, Runtime> ProvideRuntimeType<Context> for ProvideRuntimeField<Field>
where
    Context: HasField<Field, Field = Runtime> + Async,
    Runtime: HasErrorType,
{
    type Runtime = Runtime;
}

impl<Context, Field: Async> RuntimeGetter<Context> for ProvideRuntimeField<Field>
where
    Context: HasRuntimeType + HasField<Field, Field = Context::Runtime>,
{
    fn runtime(context: &Context) -> &Context::Runtime {
        context.get_field(PhantomData)
    }
}

pub type ProvideDefaultRuntimeField = ProvideRuntimeField<symbol!("runtime")>;
