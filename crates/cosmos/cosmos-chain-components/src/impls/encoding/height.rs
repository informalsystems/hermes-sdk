use cgp::prelude::HasErrorType;
use hermes_encoding_components::traits::encode_mut::MutEncoder;
use hermes_encoding_components::traits::types::encode_buffer::HasEncodeBufferType;
use hermes_protobuf_encoding_components::impls::encode_mut::proto_field::u64::EncodeU64ProtoField;
use ibc::core::client::types::Height;

pub struct EncodeHeight;

impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, Height> for EncodeHeight
where
    Encoding: HasEncodeBufferType + HasErrorType,
    EncodeU64ProtoField<1>: MutEncoder<Encoding, Strategy, u64>,
    EncodeU64ProtoField<2>: MutEncoder<Encoding, Strategy, u64>,
{
    fn encode_mut(
        encoding: &Encoding,
        height: &Height,
        buffer: &mut <Encoding as HasEncodeBufferType>::EncodeBuffer,
    ) -> Result<(), <Encoding as HasErrorType>::Error> {
        <EncodeU64ProtoField<1>>::encode_mut(encoding, &height.revision_number(), buffer)?;

        <EncodeU64ProtoField<2>>::encode_mut(encoding, &height.revision_height(), buffer)?;

        Ok(())
    }
}
