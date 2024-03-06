use cgp_core::prelude::*;

#[derive_component(ProtobufEncodingTypeComponent, ProvideProtobufEncodingType<Context>)]
pub trait HasProtobufEncodingType: Async {
    type Encoding: Async;
}

#[derive_component(ProtobufEncodingGetterComponent, ProtobufEncodingGetter<Context>)]
pub trait HasProtobufEncoding: HasProtobufEncodingType {
    fn encoding(&self) -> &Self::Encoding;
}
