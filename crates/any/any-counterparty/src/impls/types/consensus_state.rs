use cgp_core::Async;
use hermes_relayer_components::chain::traits::types::consensus_state::{
    ConsensusStateFieldGetter, HasConsensusStateType, ProvideConsensusStateType,
};
use hermes_relayer_components::chain::traits::types::timestamp::CanBuildUnixTimestamp;
use tendermint_proto::google::protobuf::Timestamp;

use crate::types::consensus_state::AnyConsensusState;

pub struct ProvideAnyConsensusState;

impl<Chain, Counterparty> ProvideConsensusStateType<Chain, Counterparty>
    for ProvideAnyConsensusState
where
    Chain: Async,
{
    type ConsensusState = AnyConsensusState;
}

impl<Chain, Counterparty> ConsensusStateFieldGetter<Chain, Counterparty>
    for ProvideAnyConsensusState
where
    Chain: HasConsensusStateType<Counterparty, ConsensusState = AnyConsensusState>,
    Counterparty: CanBuildUnixTimestamp,
{
    fn consensus_state_timestamp(consensus_state: &AnyConsensusState) -> Counterparty::Timestamp {
        match consensus_state {
            AnyConsensusState::Tendermint(consensus_state) => {
                let timestamp: Timestamp = consensus_state.timestamp.into();

                // FIXME: handle unwrap
                Counterparty::time_from_unix_timestamp(timestamp.seconds, timestamp.nanos as u32)
                    .unwrap()
            }
        }
    }
}
