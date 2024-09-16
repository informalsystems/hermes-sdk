use cgp::prelude::*;

use hermes_cosmos_encoding_components::components::CosmosEncodingComponents;
pub use hermes_cosmos_encoding_components::components::{
    DecodeBufferTypeComponent, EncodeBufferTypeComponent,
};
use hermes_encoding_components::impls::delegate::DelegateEncoding;
use hermes_encoding_components::impls::with_context::EncodeWithContext;
pub use hermes_encoding_components::traits::convert::ConverterComponent;
pub use hermes_encoding_components::traits::decode::DecoderComponent;
pub use hermes_encoding_components::traits::decode_mut::MutDecoderComponent;
pub use hermes_encoding_components::traits::encode::EncoderComponent;
pub use hermes_encoding_components::traits::encode_mut::MutEncoderComponent;
pub use hermes_encoding_components::traits::schema::SchemaGetterComponent;
pub use hermes_encoding_components::traits::types::encoded::EncodedTypeComponent;
pub use hermes_encoding_components::traits::types::schema::SchemaTypeComponent;
use hermes_protobuf_encoding_components::impl_type_url;
use hermes_protobuf_encoding_components::impls::any::{DecodeAsAnyProtobuf, EncodeAsAnyProtobuf};
use hermes_protobuf_encoding_components::impls::encode::buffer::EncodeProtoWithMutBuffer;
use hermes_protobuf_encoding_components::impls::via_any::EncodeViaAny;
pub use hermes_protobuf_encoding_components::traits::length::EncodedLengthGetterComponent;
use hermes_protobuf_encoding_components::types::strategy::{ViaAny, ViaProtobuf};
use ibc::core::client::types::Height;
use prost_types::Any;

use crate::impls::encode::client_state::EncodeWasmClientState;
use crate::impls::encode::consensus_state::EncodeWasmConsensusState;
use crate::types::client_state::WasmClientState;
use crate::types::consensus_state::WasmConsensusState;

define_components! {
    WasmEncodingComponents {
        [
            EncodedTypeComponent,
            EncodeBufferTypeComponent,
            DecodeBufferTypeComponent,
            SchemaTypeComponent,
        ]:
            CosmosEncodingComponents,
        ConverterComponent:
            DelegateEncoding<WasmConverterComponents>,
        [
            EncoderComponent,
            DecoderComponent,
        ]:
            DelegateEncoding<WasmEncoderComponents>,
        [
            MutEncoderComponent,
            MutDecoderComponent,
            EncodedLengthGetterComponent,
        ]:
            DelegateEncoding<WasmEncodeMutComponents>,
        SchemaGetterComponent:
            DelegateEncoding<WasmTypeUrlSchemas>,
    }
}

pub struct WasmConverterComponents;

pub struct WasmEncodeMutComponents;

pub struct WasmEncoderComponents;

delegate_components! {
    WasmConverterComponents {
        (WasmClientState, Any): EncodeAsAnyProtobuf<ViaProtobuf, EncodeWithContext>,
        (Any, WasmClientState): DecodeAsAnyProtobuf<ViaProtobuf, EncodeWithContext>,
        (WasmConsensusState, Any): EncodeAsAnyProtobuf<ViaProtobuf, EncodeWithContext>,
        (Any, WasmConsensusState): DecodeAsAnyProtobuf<ViaProtobuf, EncodeWithContext>,
    }
}

delegate_components! {
    WasmEncodeMutComponents {
        [
            (ViaProtobuf, Height),
        ]: CosmosEncodingComponents,

        (ViaProtobuf, WasmClientState):
            EncodeWasmClientState,

        (ViaProtobuf, WasmConsensusState):
            EncodeWasmConsensusState,
    }
}

delegate_components! {
    WasmEncoderComponents {
        (ViaAny, WasmClientState): EncodeViaAny<ViaProtobuf>,
        (ViaProtobuf, WasmClientState): EncodeProtoWithMutBuffer,
        (ViaAny, WasmConsensusState): EncodeViaAny<ViaProtobuf>,
        (ViaProtobuf, WasmConsensusState): EncodeProtoWithMutBuffer,
    }
}

pub struct WasmTypeUrlSchemas;

delegate_components! {
    WasmTypeUrlSchemas {
        WasmClientState: WasmClientStateUrl,
        WasmConsensusState: WasmConsensusStateUrl,
    }
}

impl_type_url!(WasmClientStateUrl, "/ibc.lightclients.wasm.v1.ClientState");

impl_type_url!(
    WasmConsensusStateUrl,
    "/ibc.lightclients.wasm.v1.ConsensusState"
);
