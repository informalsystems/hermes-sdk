use cgp_core::{Async, HasErrorType};

use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::chain::types::aliases::ClientId;
use crate::relay::traits::chains::{CanRaiseRelayChainErrors, HasRelayChains};

#[derive(Default, Clone, Copy)]
pub struct SourceTarget;

#[derive(Default, Clone, Copy)]
pub struct DestinationTarget;

pub trait ChainTarget<Relay: HasRelayChains>: Async + Default + Copy + private::Sealed {
    type TargetChain: HasIbcChainTypes<Self::CounterpartyChain> + HasErrorType;

    type CounterpartyChain: HasIbcChainTypes<Self::TargetChain> + HasErrorType;

    fn target_chain_error(e: <Self::TargetChain as HasErrorType>::Error) -> Relay::Error;

    fn counterparty_chain_error(
        e: <Self::CounterpartyChain as HasErrorType>::Error,
    ) -> Relay::Error;

    fn target_chain(relay: &Relay) -> &Self::TargetChain;

    fn counterparty_chain(relay: &Relay) -> &Self::CounterpartyChain;

    fn target_client_id(relay: &Relay) -> &ClientId<Self::TargetChain, Self::CounterpartyChain>;

    fn counterparty_client_id(
        relay: &Relay,
    ) -> &ClientId<Self::CounterpartyChain, Self::TargetChain>;
}

impl private::Sealed for SourceTarget {}
impl private::Sealed for DestinationTarget {}

impl<Relay> ChainTarget<Relay> for SourceTarget
where
    Relay: CanRaiseRelayChainErrors,
{
    type TargetChain = Relay::SrcChain;

    type CounterpartyChain = Relay::DstChain;

    fn target_chain_error(e: <Self::TargetChain as HasErrorType>::Error) -> Relay::Error {
        Relay::raise_error(e)
    }

    fn counterparty_chain_error(
        e: <Self::CounterpartyChain as HasErrorType>::Error,
    ) -> Relay::Error {
        Relay::raise_error(e)
    }

    fn target_chain(context: &Relay) -> &Self::TargetChain {
        context.src_chain()
    }

    fn counterparty_chain(context: &Relay) -> &Self::CounterpartyChain {
        context.dst_chain()
    }

    fn target_client_id(context: &Relay) -> &ClientId<Self::TargetChain, Self::CounterpartyChain> {
        context.src_client_id()
    }

    fn counterparty_client_id(
        context: &Relay,
    ) -> &ClientId<Self::CounterpartyChain, Self::TargetChain> {
        context.dst_client_id()
    }
}

impl<Relay> ChainTarget<Relay> for DestinationTarget
where
    Relay: CanRaiseRelayChainErrors,
{
    type TargetChain = Relay::DstChain;

    type CounterpartyChain = Relay::SrcChain;

    fn target_chain_error(e: <Self::TargetChain as HasErrorType>::Error) -> Relay::Error {
        Relay::raise_error(e)
    }

    fn counterparty_chain_error(
        e: <Self::CounterpartyChain as HasErrorType>::Error,
    ) -> Relay::Error {
        Relay::raise_error(e)
    }

    fn target_chain(context: &Relay) -> &Self::TargetChain {
        context.dst_chain()
    }

    fn counterparty_chain(context: &Relay) -> &Self::CounterpartyChain {
        context.src_chain()
    }

    fn target_client_id(context: &Relay) -> &ClientId<Self::TargetChain, Self::CounterpartyChain> {
        context.dst_client_id()
    }

    fn counterparty_client_id(
        context: &Relay,
    ) -> &ClientId<Self::CounterpartyChain, Self::TargetChain> {
        context.src_client_id()
    }
}

mod private {
    pub trait Sealed {}
}
