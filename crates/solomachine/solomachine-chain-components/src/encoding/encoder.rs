use cgp::core::component::UseContext;
use hermes_encoding_components::impls::ConvertAndEncode;
use hermes_prelude::*;
use hermes_protobuf_encoding_components::impls::any::EncodeAsAnyProtobuf;
use hermes_protobuf_encoding_components::impls::protobuf::EncodeAsProtobuf;
use hermes_protobuf_encoding_components::impls::via_any::EncodeViaAny;
use hermes_protobuf_encoding_components::types::strategy::{ViaAny, ViaProtobuf};
use prost_types::Any;

use crate::types::client_state::{ProtoSolomachineClientState, SolomachineClientState};
use crate::types::consensus_state::{ProtoSolomachineConsensusState, SolomachineConsensusState};

pub struct SolomachineEncoderComponents;

delegate_components! {
    SolomachineEncoderComponents {
        [
            (SolomachineClientState, Any),
            (SolomachineConsensusState, Any),
        ]: EncodeAsAnyProtobuf<ViaProtobuf, UseContext>,
        (ViaAny, SolomachineClientState): EncodeViaAny<ViaProtobuf>,
        (ViaProtobuf, SolomachineClientState): ConvertAndEncode<ProtoSolomachineClientState>,
        (ViaProtobuf, ProtoSolomachineClientState): EncodeAsProtobuf,

        (ViaAny, SolomachineConsensusState): EncodeViaAny<ViaProtobuf>,
        (ViaProtobuf, SolomachineConsensusState): ConvertAndEncode<ProtoSolomachineConsensusState>,
        (ViaProtobuf, ProtoSolomachineConsensusState): EncodeAsProtobuf,
    }
}
