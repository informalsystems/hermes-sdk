use cgp::prelude::*;
use hermes_encoding_components::traits::{
    HasEncodeBufferType, MutDecoder, MutDecoderComponent, MutEncoder, MutEncoderComponent,
};
use prost::bytes::BufMut;

use crate::impls::encode_mut::chunk::{HasProtoChunksDecodeBuffer, InvalidWireType, ProtoChunks};
use crate::impls::encode_mut::proto_field::length_delim::EncodeLengthDelimitedHeader;

pub struct EncodeByteField<const TAG: u32>;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy, Value, const TAG: u32> MutEncoder<Encoding, Strategy, Value>
    for EncodeByteField<TAG>
where
    Encoding: HasEncodeBufferType<EncodeBuffer: BufMut> + HasAsyncErrorType,
    EncodeLengthDelimitedHeader<TAG>: MutEncoder<Encoding, Strategy, u64>,
    Value: AsRef<[u8]>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &Value,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        let bytes = value.as_ref();
        if !bytes.is_empty() {
            <EncodeLengthDelimitedHeader<TAG>>::encode_mut(
                encoding,
                &(bytes.len() as u64),
                buffer,
            )?;
            buffer.put(bytes);
        }

        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy, Value, const TAG: u32> MutDecoder<Encoding, Strategy, Value>
    for EncodeByteField<TAG>
where
    Encoding: HasProtoChunksDecodeBuffer
        + CanRaiseAsyncError<InvalidWireType>
        + for<'a> CanRaiseAsyncError<<Value as TryFrom<&'a [u8]>>::Error>,
    Value: Default + for<'a> TryFrom<&'a [u8]>,
{
    fn decode_mut(
        _encoding: &Encoding,
        chunks: &mut ProtoChunks<'_>,
    ) -> Result<Value, Encoding::Error> {
        match chunks.get(&TAG) {
            Some(chunk) => {
                let bytes = chunk.to_length_delimited().map_err(Encoding::raise_error)?;
                Value::try_from(bytes).map_err(Encoding::raise_error)
            }
            None => Ok(Value::default()),
        }
    }
}
