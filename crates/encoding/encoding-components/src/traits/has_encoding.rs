use cgp::prelude::*;

#[cgp_component {
  name: EncodingTypeComponent,
  provider: ProvideEncodingType,
}]
pub trait HasEncodingType<Kind>: Async {
    type Encoding: Async;
}

#[cgp_component {
  name: EncodingGetterComponent,
  provider: EncodingGetter,
}]
pub trait HasEncoding<Kind>: HasEncodingType<Kind> {
    fn encoding(&self) -> &Self::Encoding;
}

#[cgp_component {
  name: DefaultEncodingGetterComponent,
  provider: DefaultEncodingGetter,
}]
pub trait HasDefaultEncoding<Kind>: HasEncodingType<Kind, Encoding: 'static> {
    fn default_encoding() -> &'static Self::Encoding;
}
