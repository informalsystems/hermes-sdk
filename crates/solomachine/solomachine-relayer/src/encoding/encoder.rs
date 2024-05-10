use cgp_core::prelude::*;
use hermes_encoding_components::impls::convert_and_encode::ConvertAndEncode;
use hermes_protobuf_encoding_components::impls::protobuf::EncodeAsProtobuf;
use hermes_protobuf_encoding_components::impls::via_any::EncodeViaAny;
use hermes_protobuf_encoding_components::types::{Any, Protobuf};

use crate::types::client_state::{ProtoSolomachineClientState, SolomachineClientState};
use crate::types::consensus_state::{ProtoSolomachineConsensusState, SolomachineConsensusState};

pub struct SolomachineEncoderComponents;

delegate_components! {
    SolomachineEncoderComponents {
        (Any, SolomachineClientState): EncodeViaAny<Protobuf>,
        (Protobuf, SolomachineClientState): ConvertAndEncode<ProtoSolomachineClientState>,
        (Protobuf, ProtoSolomachineClientState): EncodeAsProtobuf,

        (Any, SolomachineConsensusState): EncodeViaAny<Protobuf>,
        (Protobuf, SolomachineConsensusState): ConvertAndEncode<ProtoSolomachineConsensusState>,
        (Protobuf, ProtoSolomachineConsensusState): EncodeAsProtobuf,
    }
}
