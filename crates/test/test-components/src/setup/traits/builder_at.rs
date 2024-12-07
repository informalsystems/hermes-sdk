use core::marker::PhantomData;

use cgp::core::field::impls::use_field::UseField;
use cgp::prelude::*;

use crate::driver::traits::types::builder_at::{BuilderTypeAt, HasBuilderTypeAt};

#[derive_component(BuilderAtComponent, ProvideBuilderAt<Context>)]
pub trait HasBuilderAt<A: Async, B: Async>: HasBuilderTypeAt<A, B> {
    fn builder(&self) -> &BuilderTypeAt<Self, A, B>;
}

impl<Context, A: Async, B: Async, Tag> ProvideBuilderAt<Context, A, B> for UseField<Tag>
where
    Context: HasBuilderTypeAt<A, B> + HasField<Tag, Value = Context::Builder>,
{
    fn builder(context: &Context) -> &Context::Builder {
        context.get_field(PhantomData)
    }
}
