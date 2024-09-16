use cgp::prelude::*;
use hermes_encoding_components::impls::delegate::DelegateEncoding;
pub use hermes_protobuf_encoding_components::components::{
    DecodeBufferTypeComponent, DecoderComponent, EncodeBufferTypeComponent,
    EncodedLengthGetterComponent, EncodedTypeComponent, EncoderComponent, MutDecoderComponent,
    MutEncoderComponent, ProtobufEncodingComponents, SchemaTypeComponent,
};
use hermes_protobuf_encoding_components::types::strategy::ViaProtobuf;
use ibc::core::client::types::Height;
use prost_types::Any;

use crate::impls::height::EncodeHeight;

define_components! {
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
            DelegateEncoding<CosmosEncoderComponents>,
        [
            MutEncoderComponent,
            MutDecoderComponent,
            EncodedLengthGetterComponent,
        ]:
            DelegateEncoding<CosmosEncodeMutComponents>,
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
    }
}
