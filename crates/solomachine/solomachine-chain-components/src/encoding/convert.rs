use cgp::prelude::*;
use hermes_encoding_components::impls::convert::{ConvertFrom, TryConvertFrom};
use hermes_encoding_components::impls::with_context::EncodeWithContext;
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
        (SolomachineClientState, Any): EncodeAsAnyProtobuf<ViaProtobuf, EncodeWithContext>,
        (Any, SolomachineClientState): DecodeAsAnyProtobuf<ViaProtobuf, EncodeWithContext>,

        (SolomachineConsensusState, ProtoSolomachineConsensusState): ConvertFrom,
        (ProtoSolomachineConsensusState, SolomachineConsensusState): TryConvertFrom,
        (SolomachineConsensusState, Any): EncodeAsAnyProtobuf<ViaProtobuf, EncodeWithContext>,
        (Any, SolomachineConsensusState): DecodeAsAnyProtobuf<ViaProtobuf, EncodeWithContext>,
    }
}
