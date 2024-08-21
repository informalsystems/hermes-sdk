use cgp_core::prelude::*;
use hermes_encoding_components::impls::convert_and_encode::ConvertAndEncode;
use hermes_protobuf_encoding_components::impls::protobuf::EncodeAsProtobuf;
use hermes_protobuf_encoding_components::impls::via_any::EncodeViaAny;
use hermes_protobuf_encoding_components::types::{Any, ViaProtobuf};

use crate::types::client_state::{ProtoSolomachineClientState, SolomachineClientState};
use crate::types::consensus_state::{ProtoSolomachineConsensusState, SolomachineConsensusState};

pub struct SolomachineEncoderComponents;

delegate_components! {
    SolomachineEncoderComponents {
        (Any, SolomachineClientState): EncodeViaAny<ViaProtobuf>,
        (ViaProtobuf, SolomachineClientState): ConvertAndEncode<ProtoSolomachineClientState>,
        (ViaProtobuf, ProtoSolomachineClientState): EncodeAsProtobuf,

        (Any, SolomachineConsensusState): EncodeViaAny<ViaProtobuf>,
        (ViaProtobuf, SolomachineConsensusState): ConvertAndEncode<ProtoSolomachineConsensusState>,
        (ViaProtobuf, ProtoSolomachineConsensusState): EncodeAsProtobuf,
    }
}
