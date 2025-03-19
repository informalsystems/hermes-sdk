use cgp::core::field::UseField;
use cgp::prelude::*;

use crate::driver::traits::types::builder_at::{BuilderAt, HasBuilderTypeAt};

#[cgp_getter {
    provider: BuilderAtGetter,
}]
pub trait HasBuilderAt<A, B>: HasBuilderTypeAt<A, B> {
    fn builder(&self) -> &BuilderAt<Self, A, B>;
}
