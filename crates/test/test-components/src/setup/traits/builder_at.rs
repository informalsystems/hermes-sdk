use core::marker::PhantomData;

use cgp::core::field::impls::use_field::UseField;
use cgp::prelude::*;

use crate::driver::traits::types::builder_at::{BuilderTypeAt, HasBuilderTypeAt};

#[derive_component(BuilderAtComponent, ProvideBuilderAt<Context>)]
pub trait HasBuilderAt<const A: usize, const B: usize>: HasBuilderTypeAt<A, B> {
    fn builder(&self) -> &BuilderTypeAt<Self, A, B>;
}

impl<Context, const A: usize, const B: usize, Tag> ProvideBuilderAt<Context, A, B> for UseField<Tag>
where
    Context: HasBuilderTypeAt<A, B> + HasField<Tag, Field = Context::Builder>,
{
    fn builder(context: &Context) -> &Context::Builder {
        context.get_field(PhantomData)
    }
}
