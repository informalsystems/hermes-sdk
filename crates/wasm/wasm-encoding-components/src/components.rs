#[cgp::re_export_imports]
mod preset {
    use cgp::core::component::{UseContext, UseDelegate};
    use cgp::prelude::*;
    use hermes_core::encoding_components::traits::{
        ConverterComponent, DecodeBufferTypeComponent, DecoderComponent, EncodeBufferTypeComponent,
        EncodedTypeComponent, EncoderComponent, MutDecoderComponent, MutEncoderComponent,
        SchemaGetterComponent, SchemaTypeComponent,
    };
    use hermes_cosmos_encoding_components::components::CosmosEncodingComponents;
    use hermes_protobuf_encoding_components::impl_type_url;
    use hermes_protobuf_encoding_components::impls::any::{
        DecodeAsAnyProtobuf, EncodeAsAnyProtobuf,
    };
    use hermes_protobuf_encoding_components::impls::encode::buffer::EncodeProtoWithMutBuffer;
    use hermes_protobuf_encoding_components::impls::via_any::EncodeViaAny;
    use hermes_protobuf_encoding_components::traits::EncodedLengthGetterComponent;
    use hermes_protobuf_encoding_components::types::strategy::{ViaAny, ViaProtobuf};
    use ibc::clients::wasm_types::client_message::WASM_CLIENT_MESSAGE_TYPE_URL;
    use ibc::core::client::types::Height;
    use prost_types::Any;

    use crate::impls::{EncodeWasmClientMessage, EncodeWasmClientState, EncodeWasmConsensusState};
    use crate::types::{WasmClientMessage, WasmClientState, WasmConsensusState};

    cgp_preset! {
        WasmEncodingComponents {
            [
                EncodedTypeComponent,
                EncodeBufferTypeComponent,
                DecodeBufferTypeComponent,
                SchemaTypeComponent,
            ]:
                CosmosEncodingComponents::Provider,
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
            ]: CosmosEncodingComponents::Provider,

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
