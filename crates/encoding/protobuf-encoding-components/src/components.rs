use cgp::prelude::*;
use hermes_encoding_components::impls::delegate::DelegateEncoding;
use hermes_encoding_components::impls::types::encoded::ProvideEncodedBytes;
use hermes_encoding_components::impls::types::schema::ProvideStringSchema;
pub use hermes_encoding_components::traits::decode::DecoderComponent;
pub use hermes_encoding_components::traits::decode_mut::MutDecoderComponent;
pub use hermes_encoding_components::traits::encode::EncoderComponent;
pub use hermes_encoding_components::traits::encode_mut::MutEncoderComponent;
pub use hermes_encoding_components::traits::types::decode_buffer::DecodeBufferTypeComponent;
pub use hermes_encoding_components::traits::types::encode_buffer::EncodeBufferTypeComponent;
pub use hermes_encoding_components::traits::types::encoded::EncodedTypeComponent;
pub use hermes_encoding_components::traits::types::schema::SchemaTypeComponent;
use prost_types::Any;

use crate::impls::encode::buffer::EncodeProtoWithMutBuffer;
use crate::impls::encode_mut::any::EncodeAny;
use crate::impls::types::decode_buffer::ProvideProtoChunksDecodeBuffer;
use crate::impls::types::encode_buffer::ProvideBytesEncodeBuffer;
pub use crate::traits::length::EncodedLengthGetterComponent;
use crate::types::strategy::ViaProtobuf;

define_components! {
    ProtobufEncodingComponents {
        EncodedTypeComponent:
            ProvideEncodedBytes,
        EncodeBufferTypeComponent:
            ProvideBytesEncodeBuffer,
        DecodeBufferTypeComponent:
            ProvideProtoChunksDecodeBuffer,
        SchemaTypeComponent:
            ProvideStringSchema,
        [
            EncoderComponent,
            DecoderComponent,
        ]:
            DelegateEncoding<ProtobufEncoderComponents>,
        [
            MutEncoderComponent,
            MutDecoderComponent,
            EncodedLengthGetterComponent,
        ]:
            DelegateEncoding<ProtobufEncodeMutComponents>,
    }
}

pub struct ProtobufEncoderComponents;

pub struct ProtobufEncodeMutComponents;

delegate_components! {
    ProtobufEncoderComponents {
        (ViaProtobuf, Any): EncodeProtoWithMutBuffer,
    }
}

delegate_components! {
    ProtobufEncodeMutComponents {
        (ViaProtobuf, Any): EncodeAny,
    }
}
