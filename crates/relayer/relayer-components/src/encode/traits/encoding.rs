use cgp_core::prelude::*;

#[derive_component(EncodingComponent, ProvideEncoding<Context>)]
pub trait HasEncoding: Async {
    type Encoding: Async;

    fn encoding(&self) -> &Self::Encoding;
}
