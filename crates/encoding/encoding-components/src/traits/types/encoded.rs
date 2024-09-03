use cgp::prelude::*;

#[derive_component(EncodedTypeComponent, ProvideEncodedType<Encoding>)]
pub trait HasEncodedType: Async {
    type Encoded: Async;
}
