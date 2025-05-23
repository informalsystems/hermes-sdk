use core::marker::PhantomData;

use hermes_encoding_components::traits::{HasEncodeBufferType, MutEncoder, MutEncoderComponent};
use hermes_prelude::*;

use crate::impls::EncodeLengthDelimitedHeader;

pub struct EncodeLengthDelimitedProtoField<const TAG: u32, InEncoder>(pub PhantomData<InEncoder>);

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy, Value, InEncoder, const TAG: u32> MutEncoder<Encoding, Strategy, Value>
    for EncodeLengthDelimitedProtoField<TAG, InEncoder>
where
    Encoding: HasEncodeBufferType<EncodeBuffer = Vec<u8>> + HasAsyncErrorType,
    InEncoder: MutEncoder<Encoding, Strategy, Value>,
    EncodeLengthDelimitedHeader<TAG>: MutEncoder<Encoding, Strategy, u64>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &Value,
        buffer: &mut Vec<u8>,
    ) -> Result<(), Encoding::Error> {
        let mut in_buffer = Vec::new();

        InEncoder::encode_mut(encoding, value, &mut in_buffer)?;

        <EncodeLengthDelimitedHeader<TAG>>::encode_mut(
            encoding,
            &(in_buffer.len() as u64),
            buffer,
        )?;

        buffer.append(&mut in_buffer);

        Ok(())
    }
}
