use cgp_core::prelude::*;
use hermes_encoding_components::impls::convert_and_encode::ConvertAndEncode;
use hermes_encoding_components::impls::delegate::DelegateEncoding;
use hermes_encoding_components::types::via::Via;
use hermes_protobuf_encoding_components::impls::protobuf::EncodeAsProtobuf;
use hermes_protobuf_encoding_components::impls::via_any::EncodeViaAny;
use hermes_protobuf_encoding_components::types::Any;
use hermes_wasm_client_components::impls::encoding::encoder::{
    EncodeViaWasmClientState, WasmEncoderComponents,
};
use hermes_wasm_client_components::types::client_state::{ProtoWasmClientState, WasmClientState};
use ibc_proto_sov::ibc::lightclients::sovereign::tendermint::v1::ClientState as ProtoSovereignClientState;

use crate::sovereign::types::client_state::SovereignClientState;

pub struct SovereignEncoderComponents;

delegate_components! {
    SovereignEncoderComponents {
        Via<WasmClientState, SovereignClientState>: EncodeViaWasmClientState,
        Via<Any, SovereignClientState>: EncodeViaAny,
        SovereignClientState: ConvertAndEncode<ProtoSovereignClientState>,
        ProtoSovereignClientState: EncodeAsProtobuf,
        [
            Via<Any, WasmClientState>,
            WasmClientState,
            ProtoWasmClientState,
        ]:
            DelegateEncoding<WasmEncoderComponents>,
    }
}
