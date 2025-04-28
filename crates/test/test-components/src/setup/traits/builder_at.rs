use cgp::core::field::UseField;
use hermes_prelude::*;

use crate::driver::traits::{BuilderAt, HasBuilderTypeAt};

#[cgp_getter {
    name: BuilderAtGetterComponent<A, B>,
    provider: BuilderAtGetter,
}]
pub trait HasBuilderAt<A, B>: HasBuilderTypeAt<A, B> {
    fn builder(&self) -> &BuilderAt<Self, A, B>;
}
