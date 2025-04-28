use hermes_prelude::*;

#[cgp_type {
    name: EncodingTypeProviderComponent<Kind>
}]
pub trait HasEncodingType<Kind>: Async {
    type Encoding: Async;
}

#[cgp_component {
    name: EncodingGetterComponent<Kind>,
    provider: EncodingGetter,
}]
pub trait HasEncoding<Kind>: HasEncodingType<Kind> {
    fn encoding(&self) -> &Self::Encoding;
}

#[cgp_component {
    name: DefaultEncodingGetterComponent<Kind>,
    provider: DefaultEncodingGetter,
}]
pub trait HasDefaultEncoding<Kind>: HasEncodingType<Kind, Encoding: 'static> {
    fn default_encoding() -> &'static Self::Encoding;
}
