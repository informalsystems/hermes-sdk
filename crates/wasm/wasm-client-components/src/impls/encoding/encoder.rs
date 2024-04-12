use cgp_core::prelude::*;
use hermes_encoding_components::impls::convert_and_encode::ConvertAndEncode;
use hermes_encoding_components::types::via::Via;
use hermes_protobuf_encoding_components::impls::protobuf::EncodeAsProtobuf;
use hermes_protobuf_encoding_components::impls::via_any::EncodeViaAny;
use hermes_protobuf_encoding_components::types::Any;

use crate::types::client_state::{ProtoWasmClientState, WasmClientState};
use crate::types::consensus_state::{ProtoWasmConsensusState, WasmConsensusState};

pub struct WasmEncoderComponents;

delegate_components! {
    WasmEncoderComponents {
        Via<Any, WasmClientState>: EncodeViaAny,
        WasmClientState: ConvertAndEncode<ProtoWasmClientState>,
        ProtoWasmClientState: EncodeAsProtobuf,
        Via<Any, WasmConsensusState>: EncodeViaAny,
        WasmConsensusState: ConvertAndEncode<ProtoWasmConsensusState>,
        ProtoWasmConsensusState: EncodeAsProtobuf,
    }
}
