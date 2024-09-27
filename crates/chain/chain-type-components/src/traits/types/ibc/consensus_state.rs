use cgp::prelude::*;

#[derive_component(ConsensusStateTypeComponent, ProvideConsensusStateType<Chain>)]
pub trait HasConsensusStateType<Counterparty>: Async {
    /**
        The consensus state of the `Self` chain's client on the `Counterparty` chain
    */
    type ConsensusState: Async;
}
