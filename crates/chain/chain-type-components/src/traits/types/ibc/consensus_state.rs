use core::fmt::Debug;

use cgp::core::component::{UseDelegate, WithProvider};
use cgp::core::types::ProvideType;
use hermes_prelude::*;

#[cgp_component {
  name: ConsensusStateTypeComponent,
  provider: ProvideConsensusStateType,
  context: Chain,
}]
pub trait HasConsensusStateType<Counterparty>: Async {
    /**
        The consensus state of the `Self` chain's client on the `Counterparty` chain
    */
    type ConsensusState: Async + Debug;
}

pub type ConsensusStateOf<Chain, Counterparty> =
    <Chain as HasConsensusStateType<Counterparty>>::ConsensusState;

#[cgp_provider(ConsensusStateTypeComponent)]
impl<Chain, Counterparty, Components, Delegate> ProvideConsensusStateType<Chain, Counterparty>
    for UseDelegate<Components>
where
    Chain: Async,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
    Delegate: ProvideConsensusStateType<Chain, Counterparty>,
{
    type ConsensusState = Delegate::ConsensusState;
}

#[cgp_provider(ConsensusStateTypeComponent)]
impl<Chain, Counterparty, Provider, ConsensusState> ProvideConsensusStateType<Chain, Counterparty>
    for WithProvider<Provider>
where
    Chain: Async,
    Provider: ProvideType<Chain, ConsensusStateTypeComponent, Type = ConsensusState>,
    ConsensusState: Async + Debug,
{
    type ConsensusState = ConsensusState;
}
