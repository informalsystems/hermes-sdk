use cgp_core::{Async, HasErrorType};

use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::chain::types::aliases::ClientIdOf;
use crate::relay::traits::chains::{CanRaiseRelayChainErrors, HasRelayChains};
use crate::error::types::ErrorOf;

#[derive(Default, Clone, Copy)]
pub struct SourceTarget;

#[derive(Default, Clone, Copy)]
pub struct DestinationTarget;

pub trait ChainTarget<Relay: HasRelayChains>: Async + Default + Copy + private::Sealed {
    type TargetChain: HasIbcChainTypes<Self::CounterpartyChain> + HasErrorType;

    type CounterpartyChain: HasIbcChainTypes<Self::TargetChain> + HasErrorType;

    fn target_chain_error(e: ErrorOf<Self::TargetChain>) -> Relay::Error;

    fn counterparty_chain_error(e: ErrorOf<Self::CounterpartyChain>) -> Relay::Error;

    fn target_chain(relay: &Relay) -> &Self::TargetChain;

    fn counterparty_chain(relay: &Relay) -> &Self::CounterpartyChain;

    fn target_client_id(relay: &Relay) -> &ClientIdOf<Self::TargetChain, Self::CounterpartyChain>;

    fn counterparty_client_id(
        relay: &Relay,
    ) -> &ClientIdOf<Self::CounterpartyChain, Self::TargetChain>;
}

pub type TargetChainOf<Relay, Target> = <Target as ChainTarget<Relay>>::TargetChain;

pub type CounterpartyChainOf<Relay, Target> = <Target as ChainTarget<Relay>>::CounterpartyChain;

impl private::Sealed for SourceTarget {}
impl private::Sealed for DestinationTarget {}

impl<Relay> ChainTarget<Relay> for SourceTarget
where
    Relay: CanRaiseRelayChainErrors,
{
    type TargetChain = Relay::SrcChain;

    type CounterpartyChain = Relay::DstChain;

    fn target_chain_error(e: ErrorOf<Self::TargetChain>) -> Relay::Error {
        Relay::raise_error(e)
    }

    fn counterparty_chain_error(e: ErrorOf<Self::CounterpartyChain>) -> Relay::Error {
        Relay::raise_error(e)
    }

    fn target_chain(context: &Relay) -> &Self::TargetChain {
        context.src_chain()
    }

    fn counterparty_chain(context: &Relay) -> &Self::CounterpartyChain {
        context.dst_chain()
    }

    fn target_client_id(
        context: &Relay,
    ) -> &ClientIdOf<Self::TargetChain, Self::CounterpartyChain> {
        context.src_client_id()
    }

    fn counterparty_client_id(
        context: &Relay,
    ) -> &ClientIdOf<Self::CounterpartyChain, Self::TargetChain> {
        context.dst_client_id()
    }
}

impl<Relay> ChainTarget<Relay> for DestinationTarget
where
    Relay: CanRaiseRelayChainErrors,
{
    type TargetChain = Relay::DstChain;

    type CounterpartyChain = Relay::SrcChain;

    fn target_chain_error(e: ErrorOf<Self::TargetChain>) -> Relay::Error {
        Relay::raise_error(e)
    }

    fn counterparty_chain_error(e: ErrorOf<Self::CounterpartyChain>) -> Relay::Error {
        Relay::raise_error(e)
    }

    fn target_chain(context: &Relay) -> &Self::TargetChain {
        context.dst_chain()
    }

    fn counterparty_chain(context: &Relay) -> &Self::CounterpartyChain {
        context.src_chain()
    }

    fn target_client_id(
        context: &Relay,
    ) -> &ClientIdOf<Self::TargetChain, Self::CounterpartyChain> {
        context.dst_client_id()
    }

    fn counterparty_client_id(
        context: &Relay,
    ) -> &ClientIdOf<Self::CounterpartyChain, Self::TargetChain> {
        context.src_client_id()
    }
}

mod private {
    pub trait Sealed {}
}
