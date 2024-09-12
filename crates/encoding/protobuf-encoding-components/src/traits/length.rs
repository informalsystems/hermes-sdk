use cgp::prelude::*;

#[derive_component(EncodedLengthGetterComponent, EncodedLengthGetter<Encoding>)]
pub trait HasEncodedLength<Strategy, Value> {
    fn encoded_length(&self, value: &Value) -> u64;
}
