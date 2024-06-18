use std::io::Error as IoError;

use borsh::{BorshDeserialize, BorshSerialize};
use cgp_core::CanRaiseError;
use hermes_encoding_components::traits::decoder::Decoder;
use hermes_encoding_components::traits::encoded::HasEncodedType;
use hermes_encoding_components::traits::encoder::Encoder;

pub struct ViaBorsh;

pub struct EncodeWithBorsh;

impl<Encoding, Strategy, Value> Encoder<Encoding, Strategy, Value> for EncodeWithBorsh
where
    Encoding: HasEncodedType<Encoded = Vec<u8>> + CanRaiseError<IoError>,
    Value: BorshSerialize,
{
    fn encode(_encoding: &Encoding, value: &Value) -> Result<Vec<u8>, Encoding::Error> {
        let encoded = value.try_to_vec().map_err(Encoding::raise_error)?;

        Ok(encoded)
    }
}

impl<Encoding, Strategy, Value> Decoder<Encoding, Strategy, Value> for EncodeWithBorsh
where
    Encoding: HasEncodedType<Encoded = Vec<u8>> + CanRaiseError<IoError>,
    Value: BorshDeserialize,
{
    fn decode(_encoding: &Encoding, encoded: &Vec<u8>) -> Result<Value, Encoding::Error> {
        let value = Value::try_from_slice(encoded).map_err(Encoding::raise_error)?;

        Ok(value)
    }
}
