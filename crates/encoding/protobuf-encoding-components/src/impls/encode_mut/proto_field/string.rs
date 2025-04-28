use core::str::{self, Utf8Error};

use hermes_encoding_components::traits::{
    HasDecodeBufferType, MutDecoder, MutDecoderComponent, MutEncoderComponent,
};
use hermes_prelude::*;

use crate::impls::EncodeByteField;

pub struct EncodeStringField<const TAG: u32>;

delegate_components! {
    <const TAG: u32>
    EncodeStringField<TAG> {
        MutEncoderComponent: EncodeByteField<TAG>,
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy, const TAG: u32> MutDecoder<Encoding, Strategy, String>
    for EncodeStringField<TAG>
where
    Encoding: HasDecodeBufferType + HasAsyncErrorType,
    EncodeByteField<TAG>: MutDecoder<Encoding, Strategy, Utf8String>,
{
    fn decode_mut(
        encoding: &Encoding,
        chunks: &mut Encoding::DecodeBuffer<'_>,
    ) -> Result<String, Encoding::Error> {
        let Utf8String { string } = <EncodeByteField<TAG>>::decode_mut(encoding, chunks)?;
        Ok(string)
    }
}

#[derive(Default)]
pub struct Utf8String {
    pub string: String,
}

impl<'a> TryFrom<&'a [u8]> for Utf8String {
    type Error = Utf8Error;

    fn try_from(bytes: &'a [u8]) -> Result<Self, Utf8Error> {
        let string = str::from_utf8(bytes)?;

        Ok(Utf8String {
            string: string.into(),
        })
    }
}
