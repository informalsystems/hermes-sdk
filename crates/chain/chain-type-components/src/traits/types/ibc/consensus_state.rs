use cgp::core::component::{UseDelegate, WithProvider};
use cgp::core::types::traits::ProvideType;
use cgp::prelude::*;

#[derive_component(ConsensusStateTypeComponent, ProvideConsensusStateType<Chain>)]
pub trait HasConsensusStateType<Counterparty>: Async {
    /**
        The consensus state of the `Self` chain's client on the `Counterparty` chain
    */
    type ConsensusState: Async;
}

impl<Chain, Counterparty, Components, Delegate> ProvideConsensusStateType<Chain, Counterparty>
    for UseDelegate<Components>
where
    Chain: Async,
    Components: DelegateComponent<Counterparty, Delegate = Delegate>,
    Delegate: ProvideConsensusStateType<Chain, Counterparty>,
{
    type ConsensusState = Delegate::ConsensusState;
}

impl<Chain, Counterparty, Provider, ConsensusState> ProvideConsensusStateType<Chain, Counterparty>
    for WithProvider<Provider>
where
    Chain: Async,
    Provider: ProvideType<Chain, ConsensusStateTypeComponent, Type = ConsensusState>,
    ConsensusState: Async,
{
    type ConsensusState = ConsensusState;
}
