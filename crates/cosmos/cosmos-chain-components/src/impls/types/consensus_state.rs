use cgp_core::prelude::Async;
use hermes_relayer_components::chain::traits::types::consensus_state::{
    ConsensusStateFieldGetter, HasConsensusStateType, ProvideConsensusStateType,
    ProvideRawConsensusStateType,
};
use hermes_relayer_components::chain::traits::types::timestamp::CanBuildUnixTimestamp;
use prost_types::Any;
use tendermint_proto::google::protobuf::Timestamp;

use crate::types::tendermint::TendermintConsensusState;

pub struct ProvideTendermintConsensusState;

impl<Chain, Counterparty> ProvideConsensusStateType<Chain, Counterparty>
    for ProvideTendermintConsensusState
where
    Chain: Async,
{
    type ConsensusState = TendermintConsensusState;
}

impl<Chain, Counterparty> ConsensusStateFieldGetter<Chain, Counterparty>
    for ProvideTendermintConsensusState
where
    Chain: HasConsensusStateType<Counterparty, ConsensusState = TendermintConsensusState>,
    Counterparty: CanBuildUnixTimestamp,
{
    fn consensus_state_timestamp(
        consensus_state: &TendermintConsensusState,
    ) -> Counterparty::Timestamp {
        let timestamp: Timestamp = consensus_state.timestamp.into();

        // FIXME: handle unwrap
        Counterparty::time_from_unix_timestamp(timestamp.seconds, timestamp.nanos as u32).unwrap()
    }
}

pub struct ProvideAnyRawConsensusState;

impl<Chain> ProvideRawConsensusStateType<Chain> for ProvideAnyRawConsensusState
where
    Chain: Async,
{
    type RawConsensusState = Any;
}
