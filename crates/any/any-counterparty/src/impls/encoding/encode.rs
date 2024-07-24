use cgp_core::prelude::*;
use hermes_cosmos_chain_components::encoding::components::CosmosEncodingComponents;
use hermes_cosmos_chain_components::types::tendermint::{
    ProtoTendermintClientState, ProtoTendermintConsensusState, TendermintClientState,
    TendermintConsensusState,
};
use hermes_protobuf_encoding_components::types::{Any, Protobuf};

use crate::impls::encoding::client_state::EncodeAnyClientState;
use crate::impls::encoding::consensus_state::EncodeAnyConsensusState;
use crate::types::client_state::AnyClientState;
use crate::types::consensus_state::AnyConsensusState;

pub struct AnyClientEncoderComponents;

delegate_components! {
    AnyClientEncoderComponents {
        [
            (Protobuf, Any),

            (Any, TendermintClientState),
            (Protobuf, TendermintClientState),
            (Protobuf, ProtoTendermintClientState),

            (Any, TendermintConsensusState),
            (Protobuf, TendermintConsensusState),
            (Protobuf, ProtoTendermintConsensusState),
        ]:
            CosmosEncodingComponents,

        (Protobuf, AnyClientState): EncodeAnyClientState,
        (Protobuf, AnyConsensusState): EncodeAnyConsensusState,
    }
}
