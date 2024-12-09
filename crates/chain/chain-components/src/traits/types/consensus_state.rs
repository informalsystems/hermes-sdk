use cgp::core::component::UseDelegate;
use cgp::prelude::*;
pub use hermes_chain_type_components::traits::types::ibc::consensus_state::*;

use crate::traits::types::timestamp::HasTimeType;

#[cgp_component {
  name: RawConsensusStateTypeComponent,
  provider: ProvideRawConsensusStateType,
  context: Chain,
}]
pub trait HasRawConsensusStateType: Async {
    /**
        The consensus state of the `Self` chain's client on the `Counterparty` chain
    */
    type RawConsensusState: Async;
}

#[cgp_component {
  name: ConsensusStateFieldComponent,
  provider: ConsensusStateFieldGetter,
  context: Chain,
}]
pub trait HasConsensusStateFields<Counterparty>: HasConsensusStateType<Counterparty>
where
    Counterparty: HasTimeType,
{
    fn consensus_state_timestamp(consensus_state: &Self::ConsensusState) -> Counterparty::Time;
}

impl<Chain, Counterparty, Components, Delegate> ConsensusStateFieldGetter<Chain, Counterparty>
    for UseDelegate<Components>
where
    Chain: HasConsensusStateType<Counterparty>,
    Counterparty: HasTimeType,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
    Delegate: ConsensusStateFieldGetter<Chain, Counterparty>,
{
    fn consensus_state_timestamp(consensus_state: &Chain::ConsensusState) -> Counterparty::Time {
        Delegate::consensus_state_timestamp(consensus_state)
    }
}
