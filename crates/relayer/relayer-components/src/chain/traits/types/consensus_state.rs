use alloc::borrow::Cow;
use cgp_core::prelude::*;

use crate::chain::traits::types::timestamp::HasTimestampType;

#[derive_component(ConsensusStateTypeComponent, ProvideConsensusStateType<Chain>)]
pub trait HasConsensusStateType<Counterparty>: Async {
    /**
        The consensus state of the `Self` chain's client on the `Counterparty` chain
    */
    type ConsensusState: Async;
}

pub trait HasConsensusStateFields<Counterparty>: HasConsensusStateType<Counterparty>
where
    Counterparty: HasTimestampType,
{
    fn consensus_state_timestamp(
        consensus_state: &Self::ConsensusState,
    ) -> Cow<'_, Counterparty::Timestamp>;
}
