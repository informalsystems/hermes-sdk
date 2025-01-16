use core::marker::PhantomData;

use cgp::core::component::UseDelegate;
use cgp::prelude::{DelegateComponent, HasAsyncErrorType};

use crate::traits::convert::Converter;
use crate::traits::decode::Decoder;
use crate::traits::decode_mut::MutDecoder;
use crate::traits::encode::Encoder;
use crate::traits::encode_mut::MutEncoder;
use crate::traits::schema::SchemaGetter;
use crate::traits::types::decode_buffer::HasDecodeBufferType;
use crate::traits::types::encode_buffer::HasEncodeBufferType;
use crate::traits::types::encoded::HasEncodedType;
use crate::traits::types::schema::HasSchemaType;

impl<Encoding, Strategy, Value, Components, Delegate> Encoder<Encoding, Strategy, Value>
    for UseDelegate<Components>
where
    Encoding: HasEncodedType + HasAsyncErrorType,
    Components: DelegateComponent<(Strategy, Value), Delegate = Delegate>,
    Delegate: Encoder<Encoding, Strategy, Value>,
{
    fn encode(encoding: &Encoding, value: &Value) -> Result<Encoding::Encoded, Encoding::Error> {
        Delegate::encode(encoding, value)
    }
}

impl<Encoding, Strategy, Value, Components, Delegate> Decoder<Encoding, Strategy, Value>
    for UseDelegate<Components>
where
    Encoding: HasEncodedType + HasAsyncErrorType,
    Components: DelegateComponent<(Strategy, Value), Delegate = Delegate>,
    Delegate: Decoder<Encoding, Strategy, Value>,
{
    fn decode(encoding: &Encoding, encoded: &Encoding::Encoded) -> Result<Value, Encoding::Error> {
        Delegate::decode(encoding, encoded)
    }
}

impl<Encoding, From, To, Components, Delegate> Converter<Encoding, From, To>
    for UseDelegate<Components>
where
    Encoding: HasAsyncErrorType,
    Components: DelegateComponent<(From, To), Delegate = Delegate>,
    Delegate: Converter<Encoding, From, To>,
{
    fn convert(encoding: &Encoding, from: &From) -> Result<To, Encoding::Error> {
        Delegate::convert(encoding, from)
    }
}

impl<Encoding, Value, Components, Delegate> SchemaGetter<Encoding, Value>
    for UseDelegate<Components>
where
    Encoding: HasSchemaType,
    Components: DelegateComponent<Value, Delegate = Delegate>,
    Delegate: SchemaGetter<Encoding, Value>,
{
    fn schema(encoding: &Encoding, phantom: PhantomData<Value>) -> &Encoding::Schema {
        Delegate::schema(encoding, phantom)
    }
}

impl<Encoding, Strategy, Value, Components, Delegate> MutEncoder<Encoding, Strategy, Value>
    for UseDelegate<Components>
where
    Encoding: HasEncodeBufferType + HasAsyncErrorType,
    Components: DelegateComponent<(Strategy, Value), Delegate = Delegate>,
    Delegate: MutEncoder<Encoding, Strategy, Value>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &Value,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        Delegate::encode_mut(encoding, value, buffer)
    }
}

impl<Encoding, Strategy, Value, Components, Delegate> MutDecoder<Encoding, Strategy, Value>
    for UseDelegate<Components>
where
    Encoding: HasDecodeBufferType + HasAsyncErrorType,
    Components: DelegateComponent<(Strategy, Value), Delegate = Delegate>,
    Delegate: MutDecoder<Encoding, Strategy, Value>,
{
    fn decode_mut(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'_>,
    ) -> Result<Value, Encoding::Error> {
        Delegate::decode_mut(encoding, buffer)
    }
}
