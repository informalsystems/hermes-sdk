use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_encoding_components::impls::with_context::EncodeWithContext;
use hermes_encoding_components::traits::decode::Decoder;
use hermes_encoding_components::traits::encode::Encoder;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;

use crate::impls::any::{DecodeAsAnyProtobuf, EncodeAsAnyProtobuf};

pub struct EncodeViaAny<InStrategy>(pub PhantomData<InStrategy>);

impl<Encoding, Strategy, InStrategy, Value> Encoder<Encoding, Strategy, Value>
    for EncodeViaAny<InStrategy>
where
    Encoding: HasEncodedType + HasErrorType,
    EncodeAsAnyProtobuf<InStrategy, EncodeWithContext>: Encoder<Encoding, Strategy, Value>,
    InStrategy: Async,
{
    fn encode(encoding: &Encoding, value: &Value) -> Result<Encoding::Encoded, Encoding::Error> {
        <EncodeAsAnyProtobuf<InStrategy, EncodeWithContext>>::encode(encoding, value)
    }
}

impl<Encoding, Strategy, InStrategy, Value> Decoder<Encoding, Strategy, Value>
    for EncodeViaAny<InStrategy>
where
    Encoding: HasEncodedType + HasErrorType,
    DecodeAsAnyProtobuf<InStrategy, EncodeWithContext>: Decoder<Encoding, InStrategy, Value>,
    InStrategy: Async,
{
    fn decode(encoding: &Encoding, encoded: &Encoding::Encoded) -> Result<Value, Encoding::Error> {
        let value =
            <DecodeAsAnyProtobuf<InStrategy, EncodeWithContext>>::decode(encoding, encoded)?;

        Ok(value)
    }
}
