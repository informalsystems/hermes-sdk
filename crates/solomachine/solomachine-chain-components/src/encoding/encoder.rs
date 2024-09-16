use cgp::prelude::*;
use hermes_encoding_components::impls::encode::convert_and_encode::ConvertAndEncode;
use hermes_protobuf_encoding_components::impls::protobuf::EncodeAsProtobuf;
use hermes_protobuf_encoding_components::impls::via_any::EncodeViaAny;
use hermes_protobuf_encoding_components::types::strategy::{ViaAny, ViaProtobuf};

use crate::types::client_state::{ProtoSolomachineClientState, SolomachineClientState};
use crate::types::consensus_state::{ProtoSolomachineConsensusState, SolomachineConsensusState};

pub struct SolomachineEncoderComponents;

delegate_components! {
    SolomachineEncoderComponents {
        (ViaAny, SolomachineClientState): EncodeViaAny<ViaProtobuf>,
        (ViaProtobuf, SolomachineClientState): ConvertAndEncode<ProtoSolomachineClientState>,
        (ViaProtobuf, ProtoSolomachineClientState): EncodeAsProtobuf,

        (ViaAny, SolomachineConsensusState): EncodeViaAny<ViaProtobuf>,
        (ViaProtobuf, SolomachineConsensusState): ConvertAndEncode<ProtoSolomachineConsensusState>,
        (ViaProtobuf, ProtoSolomachineConsensusState): EncodeAsProtobuf,
    }
}
