use core::num::TryFromIntError;

use cgp::prelude::*;
use hermes_encoding_components::impls::encode_mut::pair::EncoderPair;
use hermes_encoding_components::traits::decode_mut::{MutDecoder, MutDecoderComponent};
use hermes_encoding_components::traits::encode_mut::{MutEncoder, MutEncoderComponent};
use hermes_encoding_components::traits::types::decode_buffer::HasDecodeBufferType;
use hermes_encoding_components::traits::types::encode_buffer::HasEncodeBufferType;
use hermes_protobuf_encoding_components::impls::encode_mut::proto_field::u64::EncodeU64ProtoField;
use ibc::core::primitives::{Timestamp, TimestampError};
use ibc_proto::google::protobuf::Timestamp as ProtoTimestamp;

pub struct EncodeTimestamp;

#[cgp_provider(MutEncoderComponent)]
impl<Encoding, Strategy> MutEncoder<Encoding, Strategy, Timestamp> for EncodeTimestamp
where
    Encoding: HasEncodeBufferType + HasAsyncErrorType,
    EncoderPair<EncodeU64ProtoField<1>, EncodeU64ProtoField<2>>:
        MutEncoder<Encoding, Strategy, (i64, i32)>,
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

        let proto_timestamp = ProtoTimestamp::from(*value);

        EncoderPair::encode_mut(
            encoding,
            &(proto_timestamp.seconds, proto_timestamp.nanos),
            buffer,
        )?;

        Ok(())
    }
}

#[cgp_provider(MutDecoderComponent)]
impl<Encoding, Strategy> MutDecoder<Encoding, Strategy, Timestamp> for EncodeTimestamp
where
    Encoding: HasDecodeBufferType
        + CanRaiseAsyncError<TryFromIntError>
        + CanRaiseAsyncError<TimestampError>,
    EncoderPair<EncodeU64ProtoField<1>, EncodeU64ProtoField<2>>:
        MutDecoder<Encoding, Strategy, (i64, i32)>,
{
    fn decode_mut(
        encoding: &Encoding,
        buffer: &mut Encoding::DecodeBuffer<'_>,
    ) -> Result<Timestamp, Encoding::Error> {
        let (seconds, nanos) = EncoderPair::decode_mut(encoding, buffer)?;

        let timestamp = Timestamp::from_unix_timestamp(
            seconds.try_into().map_err(Encoding::raise_error)?,
            nanos.try_into().map_err(Encoding::raise_error)?,
        )
        .map_err(Encoding::raise_error)?;

        Ok(timestamp)
    }
}
