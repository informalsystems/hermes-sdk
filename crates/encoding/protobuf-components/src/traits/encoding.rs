use cgp_core::prelude::*;

#[derive_component(ProtobufEncodingComponent, ProvideProtobufEncoding<Context>)]
pub trait HasProtobufEncoding: Async {
    type Encoding: Async;

    fn encoding(&self) -> &Self::Encoding;
}
