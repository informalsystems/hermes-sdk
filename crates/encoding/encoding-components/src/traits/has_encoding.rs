use cgp::prelude::*;

#[derive_component(EncodingTypeComponent, ProvideEncodingType<Context>)]
pub trait HasEncodingType<Kind>: Async {
    type Encoding: Async;
}

#[derive_component(EncodingGetterComponent, EncodingGetter<Context>)]
pub trait HasEncoding<Kind>: HasEncodingType<Kind> {
    fn encoding(&self) -> &Self::Encoding;
}

#[derive_component(DefaultEncodingGetterComponent, DefaultEncodingGetter<Context>)]
pub trait HasDefaultEncoding<Kind>: HasEncodingType<Kind, Encoding: 'static> {
    fn default_encoding() -> &'static Self::Encoding;
}
