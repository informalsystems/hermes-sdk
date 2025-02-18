#[cgp::re_export_imports]
mod preset {
    use cgp::core::component::UseDelegate;
    use cgp::prelude::*;
    use hermes_encoding_components::impls::types::encoded::ProvideEncodedBytes;
    use hermes_encoding_components::impls::types::schema::ProvideStringSchema;
    use hermes_encoding_components::traits::decode::DecoderComponent;
    use hermes_encoding_components::traits::decode_mut::MutDecoderComponent;
    use hermes_encoding_components::traits::encode::EncoderComponent;
    use hermes_encoding_components::traits::encode_mut::MutEncoderComponent;
    use hermes_encoding_components::traits::types::decode_buffer::DecodeBufferTypeComponent;
    use hermes_encoding_components::traits::types::encode_buffer::EncodeBufferTypeComponent;
    use hermes_encoding_components::traits::types::encoded::EncodedTypeComponent;
    use hermes_encoding_components::traits::types::schema::SchemaTypeComponent;
    use prost_types::Any;

    use crate::impls::encode::buffer::EncodeProtoWithMutBuffer;
    use crate::impls::encode_mut::any::EncodeAny;
    use crate::impls::types::decode_buffer::ProvideProtoChunksDecodeBuffer;
    use crate::impls::types::encode_buffer::ProvideBytesEncodeBuffer;
    use crate::traits::length::EncodedLengthGetterComponent;
    use crate::types::strategy::ViaProtobuf;

    cgp_preset! {
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
                UseDelegate<ProtobufEncoderComponents>,
            [
                MutEncoderComponent,
                MutDecoderComponent,
                EncodedLengthGetterComponent,
            ]:
                UseDelegate<ProtobufEncodeMutComponents>,
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
}
