use core::marker::PhantomData;

use cgp::core::component::UseContext;
use hermes_encoding_components::traits::{
    Decoder, DecoderComponent, Encoder, EncoderComponent, HasEncodedType,
};
use hermes_prelude::*;

use crate::impls::{DecodeAsAnyProtobuf, EncodeAsAnyProtobuf};

pub struct EncodeViaAny<InStrategy>(pub PhantomData<InStrategy>);

#[cgp_provider(EncoderComponent)]
impl<Encoding, Strategy, InStrategy, Value> Encoder<Encoding, Strategy, Value>
    for EncodeViaAny<InStrategy>
where
    Encoding: HasEncodedType + HasAsyncErrorType,
    EncodeAsAnyProtobuf<InStrategy, UseContext>: Encoder<Encoding, Strategy, Value>,
    InStrategy: Async,
{
    fn encode(encoding: &Encoding, value: &Value) -> Result<Encoding::Encoded, Encoding::Error> {
        <EncodeAsAnyProtobuf<InStrategy, UseContext>>::encode(encoding, value)
    }
}

#[cgp_provider(DecoderComponent)]
impl<Encoding, Strategy, InStrategy, Value> Decoder<Encoding, Strategy, Value>
    for EncodeViaAny<InStrategy>
where
    Encoding: HasEncodedType + HasAsyncErrorType,
    DecodeAsAnyProtobuf<InStrategy, UseContext>: Decoder<Encoding, InStrategy, Value>,
    InStrategy: Async,
{
    fn decode(encoding: &Encoding, encoded: &Encoding::Encoded) -> Result<Value, Encoding::Error> {
        let value = <DecodeAsAnyProtobuf<InStrategy, UseContext>>::decode(encoding, encoded)?;

        Ok(value)
    }
}
