use cgp::prelude::{CanRaiseError, HasErrorType};
use hermes_encoding_components::traits::decode_mut::MutDecoder;
use hermes_encoding_components::traits::encode_mut::MutEncoder;
use hermes_encoding_components::traits::types::decode_buffer::HasDecodeBufferType;
use hermes_encoding_components::traits::types::encode_buffer::HasEncodeBufferType;
use hermes_protobuf_encoding_components::impls::encode_mut::message::EncodeProstMessage;
use ibc::core::primitives::{Timestamp, TimestampError};
use ibc_proto::google::protobuf::Timestamp as ProtoTimestamp;

pub struct EncodeTimestamp;

impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, Timestamp> for EncodeTimestamp
where
    Encoding: HasEncodeBufferType + HasErrorType,
    EncodeProstMessage: MutEncoder<Encoding, Strategy, ProtoTimestamp>,
{
    fn encode_mut(
        encoding: &Encoding,
        value: &Timestamp,
        buffer: &mut Encoding::EncodeBuffer,
    ) -> Result<(), Encoding::Error> {
        // We have no choice but to use ProtoTimestamp to encode for now,
        // because the Timstamp field is currently private, and it is
        // impossible to get the seconds and nanoseconds without first
        // converting it to ProtoTimestamp.

        let proto_timestamp = ProtoTimestamp::from(value.clone());

        EncodeProstMessage::encode_mut(encoding, &proto_timestamp, buffer)?;

        Ok(())
    }
}

impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, Timestamp> for EncodeTimestamp
where
    Encoding: HasDecodeBufferType + CanRaiseError<TimestampError>,
    EncodeProstMessage: MutDecoder<Encoding, Strategy, ProtoTimestamp>,
{
    fn decode_mut(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'_>,
    ) -> Result<Timestamp, Encoding::Error> {
        let proto_timestamp = EncodeProstMessage::decode_mut(encoding, buffer)?;

        let timestamp = Timestamp::try_from(proto_timestamp).map_err(Encoding::raise_error)?;

        Ok(timestamp)
    }
}
