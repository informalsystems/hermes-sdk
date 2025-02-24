#[cgp::re_export_imports]
mod preset {
    use cgp::core::component::UseDelegate;
    use cgp::prelude::*;
    use hermes_encoding_components::traits::decode::DecoderComponent;
    use hermes_encoding_components::traits::decode_mut::MutDecoderComponent;
    use hermes_encoding_components::traits::encode::EncoderComponent;
    use hermes_encoding_components::traits::encode_mut::MutEncoderComponent;
    use hermes_encoding_components::traits::types::decode_buffer::DecodeBufferTypeComponent;
    use hermes_encoding_components::traits::types::encode_buffer::EncodeBufferTypeComponent;
    use hermes_encoding_components::traits::types::encoded::EncodedTypeComponent;
    use hermes_encoding_components::traits::types::schema::SchemaTypeComponent;
    use hermes_protobuf_encoding_components::components::ProtobufEncodingComponents;
    use hermes_protobuf_encoding_components::traits::length::EncodedLengthGetterComponent;
    use hermes_protobuf_encoding_components::types::strategy::ViaProtobuf;
    use ibc::core::client::types::Height;
    use ibc::core::commitment_types::commitment::CommitmentRoot;
    use ibc::primitives::Timestamp;
    use prost_types::Any;

    use crate::impls::commitment_root::EncodeCommitmentRoot;
    use crate::impls::height::EncodeHeight;
    use crate::impls::timestamp::EncodeTimestamp;

    cgp_preset! {
        CosmosEncodingComponents {
            [
                EncodedTypeComponent,
                EncodeBufferTypeComponent,
                DecodeBufferTypeComponent,
                SchemaTypeComponent,
            ]:
                ProtobufEncodingComponents::Provider,
            [
                EncoderComponent,
                DecoderComponent,
            ]:
                UseDelegate<CosmosEncoderComponents>,
            [
                MutEncoderComponent,
                MutDecoderComponent,
                EncodedLengthGetterComponent,
            ]:
                UseDelegate<CosmosEncodeMutComponents>,
        }
    }

    pub struct CosmosEncoderComponents;

    pub struct CosmosEncodeMutComponents;

    delegate_components! {
        CosmosEncoderComponents {
            [
                (ViaProtobuf, Any),
            ]: ProtobufEncodingComponents::Provider,
        }
    }

    delegate_components! {
        CosmosEncodeMutComponents {
            [
                (ViaProtobuf, Any),
            ]: ProtobufEncodingComponents::Provider,

            (ViaProtobuf, Height):
                EncodeHeight,

            (ViaProtobuf, CommitmentRoot):
                EncodeCommitmentRoot,

            (ViaProtobuf, Timestamp):
                EncodeTimestamp,
        }
    }
}
