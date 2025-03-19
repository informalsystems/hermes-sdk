use core::marker::PhantomData;

use cgp::core::field::UseField;
use cgp::prelude::*;

use crate::driver::traits::types::builder_at::{BuilderAt, HasBuilderTypeAt};

#[cgp_component {
    name: BuilderAtGetterComponent<A, B>,
    provider: BuilderAtGetter,
}]
pub trait HasBuilderAt<A, B>: HasBuilderTypeAt<A, B> {
    fn builder(&self) -> &BuilderAt<Self, A, B>;
}

#[cgp_provider(BuilderAtGetterComponent<A, B>)]
impl<Context, A, B, Tag> BuilderAtGetter<Context, A, B> for UseField<Tag>
where
    Context: HasBuilderTypeAt<A, B> + HasField<Tag, Value = Context::Builder>,
{
    fn builder(context: &Context) -> &Context::Builder {
        context.get_field(PhantomData)
    }
}
