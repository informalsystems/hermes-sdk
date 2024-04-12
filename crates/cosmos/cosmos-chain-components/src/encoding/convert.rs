use cgp_core::prelude::*;
use hermes_encoding_components::impls::convert::{ConvertFrom, TryConvertFrom};

use crate::types::tendermint::{
    ProtoTendermintClientState, ProtoTendermintConsensusState, TendermintClientState,
    TendermintConsensusState,
};

pub struct CosmosConverterComponents;

delegate_components! {
    CosmosConverterComponents {
        (TendermintClientState, ProtoTendermintClientState): ConvertFrom,
        (ProtoTendermintClientState, TendermintClientState): TryConvertFrom,
        (TendermintConsensusState, ProtoTendermintConsensusState): ConvertFrom,
        (ProtoTendermintConsensusState, TendermintConsensusState): TryConvertFrom,
    }
}
