#[cgp::re_export_imports]
mod preset {
    use cgp::core::component::{UseContext, UseDelegate};
    use cgp::prelude::*;
    use hermes_cosmos_encoding_components::components::CosmosEncodingComponents;
    use hermes_encoding_components::traits::convert::ConverterComponent;
    use hermes_encoding_components::traits::decode::DecoderComponent;
    use hermes_encoding_components::traits::decode_mut::MutDecoderComponent;
    use hermes_encoding_components::traits::encode::EncoderComponent;
    use hermes_encoding_components::traits::encode_mut::MutEncoderComponent;
    use hermes_encoding_components::traits::schema::SchemaGetterComponent;
    use hermes_encoding_components::traits::types::decode_buffer::DecodeBufferTypeComponent;
    use hermes_encoding_components::traits::types::encode_buffer::EncodeBufferTypeComponent;
    use hermes_encoding_components::traits::types::encoded::EncodedTypeComponent;
    use hermes_encoding_components::traits::types::schema::SchemaTypeComponent;
    use hermes_protobuf_encoding_components::impl_type_url;
    use hermes_protobuf_encoding_components::impls::any::{
        DecodeAsAnyProtobuf, EncodeAsAnyProtobuf,
    };
    use hermes_protobuf_encoding_components::impls::encode::buffer::EncodeProtoWithMutBuffer;
    use hermes_protobuf_encoding_components::impls::via_any::EncodeViaAny;
    use hermes_protobuf_encoding_components::traits::length::EncodedLengthGetterComponent;
    use hermes_protobuf_encoding_components::types::strategy::{ViaAny, ViaProtobuf};
    use ibc::clients::wasm_types::client_message::WASM_CLIENT_MESSAGE_TYPE_URL;
    use ibc::core::client::types::Height;
    use prost_types::Any;

    use crate::impls::encode::client_message::EncodeWasmClientMessage;
    use crate::impls::encode::client_state::EncodeWasmClientState;
    use crate::impls::encode::consensus_state::EncodeWasmConsensusState;
    use crate::types::client_message::WasmClientMessage;
    use crate::types::client_state::WasmClientState;
    use crate::types::consensus_state::WasmConsensusState;

    cgp_preset! {
        WasmEncodingComponents {
            [
                EncodedTypeComponent,
                EncodeBufferTypeComponent,
                DecodeBufferTypeComponent,
                SchemaTypeComponent,
            ]:
                CosmosEncodingComponents,
            ConverterComponent:
                UseDelegate<WasmConverterComponents>,
            [
                EncoderComponent,
                DecoderComponent,
            ]:
                UseDelegate<WasmEncoderComponents>,
            [
                MutEncoderComponent,
                MutDecoderComponent,
                EncodedLengthGetterComponent,
            ]:
                UseDelegate<WasmEncodeMutComponents>,
            SchemaGetterComponent:
                WasmTypeUrlSchemas,
        }
    }

    pub struct WasmConverterComponents;

    pub struct WasmEncodeMutComponents;

    pub struct WasmEncoderComponents;

    delegate_components! {
        WasmConverterComponents {
            [
                (WasmClientState, Any),
                (WasmConsensusState, Any),
            ]: EncodeAsAnyProtobuf<ViaProtobuf, UseContext>,

            [
                (Any, WasmClientState),
                (Any, WasmConsensusState),
            ]: DecodeAsAnyProtobuf<ViaProtobuf, UseContext>,
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

            (ViaProtobuf, WasmClientMessage):
                EncodeWasmClientMessage,
        }
    }

    delegate_components! {
        WasmEncoderComponents {
            [
                (ViaAny, WasmClientState),
                (ViaAny, WasmConsensusState),
                (ViaAny, WasmClientMessage),
            ]: EncodeViaAny<ViaProtobuf>,

            [
                (ViaProtobuf, WasmClientState),
                (ViaProtobuf, WasmConsensusState),
                (ViaProtobuf, WasmClientMessage),
            ]: EncodeProtoWithMutBuffer,
        }
    }

    pub struct WasmTypeUrlSchemas;

    impl_type_url!(
        WasmTypeUrlSchemas,
        WasmClientState,
        "/ibc.lightclients.wasm.v1.ClientState",
    );

    impl_type_url!(
        WasmTypeUrlSchemas,
        WasmConsensusState,
        "/ibc.lightclients.wasm.v1.ConsensusState",
    );

    impl_type_url!(
        WasmTypeUrlSchemas,
        WasmClientMessage,
        WASM_CLIENT_MESSAGE_TYPE_URL,
    );
}
