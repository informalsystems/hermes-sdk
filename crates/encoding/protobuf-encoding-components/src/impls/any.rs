use core::fmt::Display;
use core::marker::PhantomData;

use cgp_core::prelude::*;
use hermes_encoding_components::traits::convert::Converter;
use hermes_encoding_components::traits::decode::Decoder;
use hermes_encoding_components::traits::encode::Encoder;
use hermes_encoding_components::traits::schema::HasSchema;
use hermes_encoding_components::traits::types::encoded::HasEncodedType;
use prost::{DecodeError, Message};
use prost_types::Any;

#[derive(Debug)]
pub struct TypeUrlMismatchError {
    pub expected_url: String,
    pub actual_url: String,
}

pub struct EncodeAsAnyProtobuf<InStrategy, InEncoder>(pub PhantomData<(InStrategy, InEncoder)>);

pub struct DecodeAsAnyProtobuf<InStrategy, InEncoder>(pub PhantomData<(InStrategy, InEncoder)>);

impl<InEncoder, Encoding, Strategy, InStrategy, Value> Encoder<Encoding, Strategy, Value>
    for EncodeAsAnyProtobuf<InStrategy, InEncoder>
where
    Encoding: HasEncodedType<Encoded = Vec<u8>> + HasSchema<Value> + HasErrorType,
    InEncoder: Encoder<Encoding, InStrategy, Value>,
    Encoding::Schema: Display,
    Self: Converter<Encoding, Value, Any>,
{
    fn encode(encoding: &Encoding, value: &Value) -> Result<Vec<u8>, Encoding::Error> {
        let any = Self::convert(encoding, value)?;

        Ok(any.encode_to_vec())
    }
}

impl<InEncoder, Encoding, InStrategy, Value> Converter<Encoding, Value, Any>
    for EncodeAsAnyProtobuf<InStrategy, InEncoder>
where
    Encoding: HasEncodedType<Encoded = Vec<u8>> + HasSchema<Value> + HasErrorType,
    InEncoder: Encoder<Encoding, InStrategy, Value>,
    Encoding::Schema: Display,
    InStrategy: Async,
{
    fn convert(encoding: &Encoding, value: &Value) -> Result<Any, Encoding::Error> {
        let encoded = InEncoder::encode(encoding, value)?;
        let type_url = encoding.schema(PhantomData::<Value>);

        let any = Any {
            value: encoded,
            type_url: type_url.to_string(),
        };

        Ok(any)
    }
}

impl<InEncoder, Encoding, Strategy, InStrategy, Value> Decoder<Encoding, Strategy, Value>
    for DecodeAsAnyProtobuf<InStrategy, InEncoder>
where
    Encoding: HasEncodedType<Encoded = Vec<u8>> + CanRaiseError<DecodeError>,
    Self: Converter<Encoding, Any, Value>,
{
    fn decode(encoding: &Encoding, encoded: &Vec<u8>) -> Result<Value, Encoding::Error> {
        let any: Any = Message::decode(encoded.as_ref()).map_err(Encoding::raise_error)?;

        Self::convert(encoding, &any)
    }
}

impl<InEncoder, Encoding, InStrategy, Value> Converter<Encoding, Any, Value>
    for DecodeAsAnyProtobuf<InStrategy, InEncoder>
where
    Encoding:
        HasEncodedType<Encoded = Vec<u8>> + HasSchema<Value> + CanRaiseError<TypeUrlMismatchError>,
    InEncoder: Decoder<Encoding, InStrategy, Value>,
    Encoding::Schema: Display,
    InStrategy: Async,
{
    fn convert(encoding: &Encoding, any: &Any) -> Result<Value, Encoding::Error> {
        let type_url = encoding.schema(PhantomData::<Value>).to_string();

        if any.type_url != type_url {
            return Err(Encoding::raise_error(TypeUrlMismatchError {
                expected_url: type_url,
                actual_url: any.type_url.clone(),
            }));
        }

        InEncoder::decode(encoding, &any.value)
    }
}
