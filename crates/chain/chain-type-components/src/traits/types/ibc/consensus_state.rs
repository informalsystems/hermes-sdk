use cgp::core::component::UseDelegate;
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
