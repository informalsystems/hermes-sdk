use hermes_cosmos_chain_components::encoding::CosmosClientEncodingComponents;
use hermes_cosmos_chain_components::types::{
    ProtoTendermintClientState, ProtoTendermintConsensusState, TendermintClientState,
    TendermintConsensusState,
};
use hermes_prelude::*;
use hermes_protobuf_encoding_components::types::any::Any;
use hermes_protobuf_encoding_components::types::strategy::ViaProtobuf;

use crate::impls::{EncodeAnyClientState, EncodeAnyConsensusState};
use crate::types::{AnyClientState, AnyConsensusState};

pub struct AnyClientEncoderComponents;

delegate_components! {
    AnyClientEncoderComponents {
        [
            (ViaProtobuf, Any),

            (Any, TendermintClientState),
            (ViaProtobuf, TendermintClientState),
            (ViaProtobuf, ProtoTendermintClientState),

            (Any, TendermintConsensusState),
            (ViaProtobuf, TendermintConsensusState),
            (ViaProtobuf, ProtoTendermintConsensusState),
        ]:
            CosmosClientEncodingComponents::Provider,

        (ViaProtobuf, AnyClientState): EncodeAnyClientState,
        (ViaProtobuf, AnyConsensusState): EncodeAnyConsensusState,
    }
}
