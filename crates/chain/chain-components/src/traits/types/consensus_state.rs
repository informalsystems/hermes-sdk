use cgp::prelude::*;

use crate::traits::types::timestamp::HasTimestampType;

#[derive_component(ConsensusStateTypeComponent, ProvideConsensusStateType<Chain>)]
pub trait HasConsensusStateType<Counterparty>: Async {
    /**
        The consensus state of the `Self` chain's client on the `Counterparty` chain
    */
    type ConsensusState: Async;
}

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
    Counterparty: HasTimestampType,
{
    fn consensus_state_timestamp(consensus_state: &Self::ConsensusState)
        -> Counterparty::Timestamp;
}
