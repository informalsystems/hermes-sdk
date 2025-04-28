#[cgp::re_export_imports]
mod preset {
    use cgp::core::component::UseDelegate;
    use hermes_encoding_components::impls::{ProvideEncodedBytes, ProvideStringSchema};
    use hermes_encoding_components::traits::{
        DecodeBufferTypeComponent, DecoderComponent, EncodeBufferTypeComponent,
        EncodedTypeComponent, EncoderComponent, MutDecoderComponent, MutEncoderComponent,
        SchemaTypeComponent,
    };
    use hermes_prelude::*;
    use prost_types::Any;

    use crate::impls::encode::buffer::EncodeProtoWithMutBuffer;
    use crate::impls::encode_mut::any::EncodeAny;
    use crate::impls::types::decode_buffer::ProvideProtoChunksDecodeBuffer;
    use crate::impls::types::encode_buffer::ProvideBytesEncodeBuffer;
    use crate::traits::EncodedLengthGetterComponent;
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
