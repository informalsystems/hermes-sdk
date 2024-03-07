use cgp_core::prelude::*;

#[derive_component(EncodingTypeComponent, ProvideEncodingType<Context>)]
pub trait HasEncodingType: Async {
    type Encoding: Async;
}

#[derive_component(EncodingGetterComponent, EncodingGetter<Context>)]
pub trait HasEncoding: HasEncodingType {
    fn encoding(&self) -> &Self::Encoding;
}

#[derive_component(DefaultEncodingGetterComponent, DefaultEncodingGetter<Context>)]
pub trait HasDefaultEncoding: HasEncodingType {
    fn default_encoding() -> Self::Encoding;
}
