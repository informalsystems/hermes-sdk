use cgp::core::component::UseContext;
use cgp::prelude::*;
use hermes_encoding_components::impls::{ConvertFrom, TryConvertFrom};
use hermes_protobuf_encoding_components::impls::any::{DecodeAsAnyProtobuf, EncodeAsAnyProtobuf};
use hermes_protobuf_encoding_components::types::any::Any;
use hermes_protobuf_encoding_components::types::strategy::ViaProtobuf;

use crate::types::client_state::{ProtoSolomachineClientState, SolomachineClientState};
use crate::types::consensus_state::{ProtoSolomachineConsensusState, SolomachineConsensusState};

pub struct SolomachineConverterComponents;

delegate_components! {
    SolomachineConverterComponents {
        (SolomachineClientState, ProtoSolomachineClientState): ConvertFrom,
        (ProtoSolomachineClientState, SolomachineClientState): TryConvertFrom,
        (SolomachineClientState, Any): EncodeAsAnyProtobuf<ViaProtobuf, UseContext>,
        (Any, SolomachineClientState): DecodeAsAnyProtobuf<ViaProtobuf, UseContext>,

        (SolomachineConsensusState, ProtoSolomachineConsensusState): ConvertFrom,
        (ProtoSolomachineConsensusState, SolomachineConsensusState): TryConvertFrom,
        (SolomachineConsensusState, Any): EncodeAsAnyProtobuf<ViaProtobuf, UseContext>,
        (Any, SolomachineConsensusState): DecodeAsAnyProtobuf<ViaProtobuf, UseContext>,
    }
}
