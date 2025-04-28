use core::marker::PhantomData;

use cgp::core::component::UseDelegate;
use hermes_prelude::*;

use crate::traits::{
    Converter, ConverterComponent, Decoder, DecoderComponent, Encoder, EncoderComponent,
    HasDecodeBufferType, HasEncodeBufferType, HasEncodedType, HasSchemaType, MutDecoder,
    MutDecoderComponent, MutEncoder, MutEncoderComponent, SchemaGetter, SchemaGetterComponent,
};

#[cgp_provider(EncoderComponent)]
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

#[cgp_provider(DecoderComponent)]
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

#[cgp_provider(ConverterComponent)]
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

#[cgp_provider(SchemaGetterComponent)]
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

#[cgp_provider(MutEncoderComponent)]
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

#[cgp_provider(MutDecoderComponent)]
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
