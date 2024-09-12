use hermes_encoding_components::traits::encode_mut::{CanEncodeMut, MutEncoder};

pub struct EncodeOption;

impl<Encoding, Strategy, Value> MutEncoder<Encoding, Strategy, Option<Value>> for EncodeOption
where
    Encoding: CanEncodeMut<Strategy, Value>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &Option<Value>,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        if let Some(value) = value {
            encoding.encode_mut(value, buffer)?;
        }

        Ok(())
    }
}
