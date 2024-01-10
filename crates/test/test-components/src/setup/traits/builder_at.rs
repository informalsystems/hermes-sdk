use crate::driver::traits::types::builder_at::{BuilderTypeAt, HasBuilderTypeAt};

pub trait HasBuilderAt<const A: usize, const B: usize>: HasBuilderTypeAt<A, B> {
    fn builder(&self) -> &BuilderTypeAt<Self, A, B>;
}
