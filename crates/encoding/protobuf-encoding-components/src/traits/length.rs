use cgp::core::component::UseDelegate;
use cgp::prelude::*;

#[cgp_component {
  name: EncodedLengthGetterComponent,
  provider: EncodedLengthGetter,
  context: Encoding,
}]
pub trait HasEncodedLength<Strategy, Value> {
    fn encoded_length(&self, value: &Value) -> u64;
}

impl<Encoding, Strategy, Value, Components> EncodedLengthGetter<Encoding, Strategy, Value>
    for UseDelegate<Components>
where
    Components: DelegateComponent<(Strategy, Value)>,
    Components::Delegate: EncodedLengthGetter<Encoding, Strategy, Value>,
{
    fn encoded_length(encoding: &Encoding, value: &Value) -> u64 {
        Components::Delegate::encoded_length(encoding, value)
    }
}
