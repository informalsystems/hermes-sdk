use cgp::prelude::{CanRaiseError, HasErrorType};
use hermes_encoding_components::impls::encode_mut::pair::EncoderPair;
use hermes_encoding_components::traits::decode_mut::MutDecoder;
use hermes_encoding_components::traits::encode_mut::MutEncoder;
use hermes_encoding_components::traits::types::decode_buffer::HasDecodeBufferType;
use hermes_encoding_components::traits::types::encode_buffer::HasEncodeBufferType;
use hermes_protobuf_encoding_components::impls::encode_mut::proto_field::u64::EncodeU64ProtoField;
use ibc::core::client::types::error::ClientError;
use ibc::core::client::types::Height;

pub struct EncodeHeight;

impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, Height> for EncodeHeight
where
    Encoding: HasEncodeBufferType + HasErrorType,
    EncoderPair<EncodeU64ProtoField<1>, EncodeU64ProtoField<2>>:
        MutEncoder<Encoding, Strategy, (u64, u64)>,
{
    fn encode_mut(
        encoding: &Encoding,
        height: &Height,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        EncoderPair::encode_mut(
            encoding,
            &(height.revision_number(), height.revision_height()),
            buffer,
        )?;

        Ok(())
    }
}

impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, Height> for EncodeHeight
where
    Encoding: HasDecodeBufferType + CanRaiseError<ClientError>,
    EncoderPair<EncodeU64ProtoField<1>, EncodeU64ProtoField<2>>:
        MutDecoder<Encoding, Strategy, (u64, u64)>,
{
    fn decode_mut(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'_>,
    ) -> Result<Height, Encoding::Error> {
        let (revision_number, revision_height) = EncoderPair::decode_mut(encoding, buffer)?;

        Height::new(revision_number, revision_height).map_err(Encoding::raise_error)
    }
}
