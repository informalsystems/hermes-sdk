use cgp_core::prelude::Async;
use hermes_relayer_components::chain::traits::types::consensus_state::{
    ConsensusFieldGetter, HasConsensusStateType, ProvideConsensusStateType,
    ProvideRawConsensusStateType,
};
use hermes_relayer_components::chain::traits::types::timestamp::HasTimestampType;
use prost_types::Any;

use crate::types::tendermint::TendermintConsensusState;

pub struct ProvideTendermintConsensusState;

impl<Chain, Counterparty> ProvideConsensusStateType<Chain, Counterparty>
    for ProvideTendermintConsensusState
where
    Chain: Async,
{
    type ConsensusState = TendermintConsensusState;
}

impl<Chain, Counterparty> ConsensusFieldGetter<Chain, Counterparty>
    for ProvideTendermintConsensusState
where
    Chain: HasConsensusStateType<Counterparty, ConsensusState = TendermintConsensusState>,
    Counterparty: HasTimestampType,
{
    fn consensus_state_timestamp(
        consensus_state: &TendermintConsensusState,
    ) -> Counterparty::Timestamp {
        // FIXME(romac): This is a temporary workaround until we have a proper conversion,
        // and can blow out if the timestamp is later than July 21st, 2554.
        let nanos = consensus_state.timestamp.unix_timestamp_nanos() as u64;
        Counterparty::timestamp_from_nanos(nanos)
    }
}

pub struct ProvideAnyRawConsensusState;

impl<Chain> ProvideRawConsensusStateType<Chain> for ProvideAnyRawConsensusState
where
    Chain: Async,
{
    type RawConsensusState = Any;
}
