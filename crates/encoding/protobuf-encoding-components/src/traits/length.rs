use cgp::prelude::*;
use hermes_encoding_components::impls::delegate::DelegateEncoding;

#[derive_component(EncodedLengthGetterComponent, EncodedLengthGetter<Encoding>)]
pub trait HasEncodedLength<Strategy, Value> {
    fn encoded_length(&self, value: &Value) -> u64;
}

impl<Encoding, Strategy, Value, Components> EncodedLengthGetter<Encoding, Strategy, Value>
    for DelegateEncoding<Components>
where
    Components: DelegateComponent<(Strategy, Value)>,
    Components::Delegate: EncodedLengthGetter<Encoding, Strategy, Value>,
{
    fn encoded_length(encoding: &Encoding, value: &Value) -> u64 {
        Components::Delegate::encoded_length(encoding, value)
    }
}
