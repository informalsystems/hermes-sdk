use cgp_core::prelude::*;
use hermes_encoding_components::impls::convert::{ConvertFrom, TryConvertFrom};
use hermes_protobuf_encoding_components::impls::any::{DecodeAsAnyProtobuf, EncodeAsAnyProtobuf};
use hermes_protobuf_encoding_components::impls::from_context::EncodeFromContext;
use hermes_protobuf_encoding_components::types::{Any, Protobuf};

use crate::types::client_state::{ProtoSolomachineClientState, SolomachineClientState};
use crate::types::consensus_state::{ProtoSolomachineConsensusState, SolomachineConsensusState};

pub struct SolomachineConverterComponents;

delegate_components! {
    SolomachineConverterComponents {
        (SolomachineClientState, ProtoSolomachineClientState): ConvertFrom,
        (ProtoSolomachineClientState, SolomachineClientState): TryConvertFrom,
        (SolomachineClientState, Any): EncodeAsAnyProtobuf<Protobuf, EncodeFromContext>,
        (Any, SolomachineClientState): DecodeAsAnyProtobuf<Protobuf, EncodeFromContext>,

        (SolomachineConsensusState, ProtoSolomachineConsensusState): ConvertFrom,
        (ProtoSolomachineConsensusState, SolomachineConsensusState): TryConvertFrom,
        (SolomachineConsensusState, Any): EncodeAsAnyProtobuf<Protobuf, EncodeFromContext>,
        (Any, SolomachineConsensusState): DecodeAsAnyProtobuf<Protobuf, EncodeFromContext>,
    }
}
