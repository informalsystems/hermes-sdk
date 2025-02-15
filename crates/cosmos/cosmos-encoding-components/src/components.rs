use cgp::core::component::UseDelegate;
use cgp::prelude::*;
pub use hermes_protobuf_encoding_components::components::{
    DecodeBufferTypeComponent, DecoderComponent, EncodeBufferTypeComponent,
    EncodedLengthGetterComponent, EncodedTypeComponent, EncoderComponent, MutDecoderComponent,
    MutEncoderComponent, ProtobufEncodingComponents, SchemaTypeComponent,
};
use hermes_protobuf_encoding_components::types::strategy::ViaProtobuf;
use ibc::core::client::types::Height;
use ibc::core::commitment_types::commitment::CommitmentRoot;
use ibc::primitives::Timestamp;
use prost_types::Any;

use crate::impls::commitment_root::EncodeCommitmentRoot;
use crate::impls::height::EncodeHeight;
use crate::impls::timestamp::EncodeTimestamp;

pub struct CosmosEncodingComponents;

delegate_components! {
    CosmosEncodingComponents {
        [
            EncodedTypeComponent,
            EncodeBufferTypeComponent,
            DecodeBufferTypeComponent,
            SchemaTypeComponent,
        ]:
            ProtobufEncodingComponents,
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
        ]: ProtobufEncodingComponents,
    }
}

delegate_components! {
    CosmosEncodeMutComponents {
        [
            (ViaProtobuf, Any),
        ]: ProtobufEncodingComponents,

        (ViaProtobuf, Height):
            EncodeHeight,

        (ViaProtobuf, CommitmentRoot):
            EncodeCommitmentRoot,

        (ViaProtobuf, Timestamp):
            EncodeTimestamp,
    }
}
