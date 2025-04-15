use cgp::prelude::*;
use hermes_cosmos_chain_components::encoding::CosmosClientEncodingComponents;
use hermes_cosmos_chain_components::types::{
    ProtoTendermintClientState, ProtoTendermintConsensusState, TendermintClientState,
    TendermintConsensusState,
};
use hermes_protobuf_encoding_components::types::any::Any;

use crate::impls::{EncodeAnyClientState, EncodeAnyConsensusState};
use crate::types::{AnyClientState, AnyConsensusState};

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
            CosmosClientEncodingComponents::Provider,

        (Any, AnyClientState): EncodeAnyClientState,
        (Any, AnyConsensusState): EncodeAnyConsensusState,
    }
}
