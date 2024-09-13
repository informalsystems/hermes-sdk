use cgp::prelude::*;
use hermes_protobuf_encoding_components::impls::encode::buffer::EncodeProtoWithMutBuffer;
use hermes_protobuf_encoding_components::impls::via_any::EncodeViaAny;
use hermes_protobuf_encoding_components::types::strategy::{ViaAny, ViaProtobuf};

use crate::types::client_state::WasmClientState;
use crate::types::consensus_state::WasmConsensusState;

pub struct WasmEncoderComponents;

delegate_components! {
    WasmEncoderComponents {
        (ViaAny, WasmClientState): EncodeViaAny<ViaProtobuf>,
        (ViaProtobuf, WasmClientState): EncodeProtoWithMutBuffer,
        (ViaAny, WasmConsensusState): EncodeViaAny<ViaProtobuf>,
        (ViaProtobuf, WasmConsensusState): EncodeProtoWithMutBuffer,
    }
}
