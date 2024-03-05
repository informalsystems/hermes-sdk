use core::marker::PhantomData;

use cgp_core::{CanRaiseError, HasErrorType};
use hermes_relayer_components::encode::decoder::Decoder;
use hermes_relayer_components::encode::encoded::HasEncodedType;
use hermes_relayer_components::encode::encoder::Encoder;
use hermes_relayer_components::encode::schema::HasSchema;
use ibc_proto::google::protobuf::Any;
use prost::{DecodeError, Message};

#[derive(Debug)]
pub struct TypeUrlMismatchError {
    pub expected_url: String,
    pub actual_url: String,
}

pub struct EncodeAsAnyProtobuf<InEncoder>(pub PhantomData<InEncoder>);

impl<InEncoder, Encoding, Value> Encoder<Encoding, Value> for EncodeAsAnyProtobuf<InEncoder>
where
    Encoding:
        HasEncodedType<Encoded = Vec<u8>> + HasSchema<Value, Schema = &'static str> + HasErrorType,
    InEncoder: Encoder<Encoding, Value>,
{
    fn encode(encoding: &Encoding, value: &Value) -> Result<Vec<u8>, Encoding::Error> {
        let encoded = InEncoder::encode(encoding, value)?;
        let type_url = *encoding.schema(PhantomData::<Value>);

        let any = Any {
            value: encoded,
            type_url: type_url.to_owned(),
        };

        Ok(any.encode_to_vec())
    }
}

impl<InEncoder, Encoding, Value> Decoder<Encoding, Value> for EncodeAsAnyProtobuf<InEncoder>
where
    Encoding: HasEncodedType<Encoded = Vec<u8>>
        + HasSchema<Value, Schema = &'static str>
        + CanRaiseError<DecodeError>
        + CanRaiseError<TypeUrlMismatchError>,
    InEncoder: Decoder<Encoding, Value>,
{
    fn decode(encoding: &Encoding, encoded: &Vec<u8>) -> Result<Value, Encoding::Error> {
        let type_url = *encoding.schema(PhantomData::<Value>);

        let any: Any = Message::decode(encoded.as_ref()).map_err(Encoding::raise_error)?;

        if &any.type_url != type_url {
            return Err(Encoding::raise_error(TypeUrlMismatchError {
                expected_url: type_url.to_owned(),
                actual_url: any.type_url,
            }));
        }

        InEncoder::decode(encoding, &any.value)
    }
}
