use crate::driver::traits::types::builder_at::{BuilderTypeAt, HasBuilderTypeAt};
use crate::setup::traits::driver::HasDriverType;

pub trait HasBuilderAt<const A: usize, const B: usize>: HasDriverType
where
    Self::Driver: HasBuilderTypeAt<A, B>,
{
    fn builder(&self) -> &BuilderTypeAt<Self::Driver, A, B>;
}
