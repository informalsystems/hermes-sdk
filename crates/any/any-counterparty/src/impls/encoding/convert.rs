use cgp::prelude::*;
use hermes_cosmos_chain_components::encoding::components::CosmosClientEncodingComponents;
use hermes_cosmos_chain_components::types::tendermint::{
    ProtoTendermintClientState, ProtoTendermintConsensusState, TendermintClientState,
    TendermintConsensusState,
};
use hermes_protobuf_encoding_components::types::any::Any;

use crate::impls::encoding::client_state::EncodeAnyClientState;
use crate::impls::encoding::consensus_state::EncodeAnyConsensusState;
use crate::types::client_state::AnyClientState;
use crate::types::consensus_state::AnyConsensusState;

pub struct AnyClientConverterComponents;

delegate_components! {
    AnyClientConverterComponents {
        [
            (TendermintClientState, ProtoTendermintClientState),
            (ProtoTendermintClientState, TendermintClientState),
            (TendermintConsensusState, ProtoTendermintConsensusState),
            (ProtoTendermintConsensusState, TendermintConsensusState),
            (TendermintClientState, Any),
            (Any, TendermintClientState),
            (TendermintConsensusState, Any),
            (Any, TendermintConsensusState),
        ]:
            CosmosClientEncodingComponents,

        (Any, AnyClientState): EncodeAnyClientState,
        (Any, AnyConsensusState): EncodeAnyConsensusState,
    }
}
