use cgp::core::error::{ErrorOf, HasErrorType};
use cgp::core::Async;

use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::chain::types::aliases::ClientIdOf;
use crate::relay::traits::chains::{CanRaiseRelayChainErrors, HasRelayChains, HasRelayClientIds};

#[derive(Default, Clone, Copy)]
pub struct SourceTarget;

#[derive(Default, Clone, Copy)]
pub struct DestinationTarget;

pub trait ChainTargetType<Relay>: Async + Default + Copy + private::Sealed {
    type TargetChain: HasIbcChainTypes<Self::CounterpartyChain> + HasErrorType;

    type CounterpartyChain: HasIbcChainTypes<Self::TargetChain> + HasErrorType;
}

pub trait ChainTarget<Relay: HasRelayChains>: ChainTargetType<Relay> {
    fn target_chain_error(e: ErrorOf<Self::TargetChain>) -> Relay::Error;

    fn counterparty_chain_error(e: ErrorOf<Self::CounterpartyChain>) -> Relay::Error;

    fn target_chain(relay: &Relay) -> &Self::TargetChain;

    fn counterparty_chain(relay: &Relay) -> &Self::CounterpartyChain;

    fn target_client_id(relay: &Relay) -> &ClientIdOf<Self::TargetChain, Self::CounterpartyChain>;

    fn counterparty_client_id(
        relay: &Relay,
    ) -> &ClientIdOf<Self::CounterpartyChain, Self::TargetChain>;
}

pub type TargetChainOf<Relay, Target> = <Target as ChainTargetType<Relay>>::TargetChain;

pub type CounterpartyChainOf<Relay, Target> = <Target as ChainTargetType<Relay>>::CounterpartyChain;

impl private::Sealed for SourceTarget {}
impl private::Sealed for DestinationTarget {}

impl<Relay> ChainTargetType<Relay> for SourceTarget
where
    Relay: HasRelayClientIds,
{
    type TargetChain = Relay::SrcChain;

    type CounterpartyChain = Relay::DstChain;
}

impl<Relay> ChainTarget<Relay> for SourceTarget
where
    Relay: HasRelayClientIds + CanRaiseRelayChainErrors,
{
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

impl<Relay> ChainTargetType<Relay> for DestinationTarget
where
    Relay: HasRelayChains,
{
    type TargetChain = Relay::DstChain;

    type CounterpartyChain = Relay::SrcChain;
}

impl<Relay> ChainTarget<Relay> for DestinationTarget
where
    Relay: HasRelayClientIds + CanRaiseRelayChainErrors,
{
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
