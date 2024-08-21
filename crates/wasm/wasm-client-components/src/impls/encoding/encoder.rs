use cgp_core::prelude::*;
use hermes_encoding_components::impls::convert_and_encode::ConvertAndEncode;
use hermes_protobuf_encoding_components::impls::protobuf::EncodeAsProtobuf;
use hermes_protobuf_encoding_components::impls::via_any::EncodeViaAny;
use hermes_protobuf_encoding_components::types::{Any, ViaProtobuf};

use crate::types::client_state::{ProtoWasmClientState, WasmClientState};
use crate::types::consensus_state::{ProtoWasmConsensusState, WasmConsensusState};

pub struct WasmEncoderComponents;

delegate_components! {
    WasmEncoderComponents {
        (Any, WasmClientState): EncodeViaAny<ViaProtobuf>,
        (ViaProtobuf, WasmClientState): ConvertAndEncode<ProtoWasmClientState>,
        (ViaProtobuf, ProtoWasmClientState): EncodeAsProtobuf,
        (Any, WasmConsensusState): EncodeViaAny<ViaProtobuf>,
        (ViaProtobuf, WasmConsensusState): ConvertAndEncode<ProtoWasmConsensusState>,
        (ViaProtobuf, ProtoWasmConsensusState): EncodeAsProtobuf,
    }
}
