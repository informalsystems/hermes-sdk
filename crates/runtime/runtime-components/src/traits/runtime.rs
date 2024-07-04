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

pub struct GetRuntimeField<Key>(pub PhantomData<Key>);

impl<Context, Key> RuntimeGetter<Context> for GetRuntimeField<Key>
where
    Context: HasRuntimeType + HasField<Key, Field = Context::Runtime>,
    Key: Async,
{
    fn runtime(context: &Context) -> &Context::Runtime {
        context.get_field(PhantomData)
    }
}
