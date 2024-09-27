use cgp::prelude::*;

use crate::traits::types::timestamp::HasTimeType;

pub use hermes_chain_type_components::traits::types::ibc::consensus_state::*;

#[derive_component(RawConsensusStateTypeComponent, ProvideRawConsensusStateType<Chain>)]
pub trait HasRawConsensusStateType: Async {
    /**
        The consensus state of the `Self` chain's client on the `Counterparty` chain
    */
    type RawConsensusState: Async;
}

#[derive_component(ConsensusStateFieldComponent, ConsensusStateFieldGetter<Chain>)]
pub trait HasConsensusStateFields<Counterparty>: HasConsensusStateType<Counterparty>
where
    Counterparty: HasTimeType,
{
    fn consensus_state_timestamp(consensus_state: &Self::ConsensusState) -> Counterparty::Time;
}
