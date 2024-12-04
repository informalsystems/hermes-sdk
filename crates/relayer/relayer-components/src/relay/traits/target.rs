use core::marker::PhantomData;

use cgp::core::error::{ErrorOf, HasErrorType};
use cgp::core::Async;
use cgp::prelude::CanRaiseError;

use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::chain::types::aliases::ClientIdOf;
use crate::multi::traits::chain_at::{HasChainAt, HasChainTypeAt};
use crate::multi::traits::client_id_at::HasClientIdAt;
use crate::multi::types::tags::{Dst, Src};
use crate::relay::traits::chains::{CanRaiseRelayChainErrors, HasRelayChains, HasRelayClientIds};

#[derive(Default, Clone, Copy)]
pub struct SourceTarget;

#[derive(Default, Clone, Copy)]
pub struct DestinationTarget;

pub trait RelayTarget: Async + Default + Copy + private::Sealed {
    type Chain: Async;

    type Counterparty: Async;
}

impl RelayTarget for SourceTarget {
    type Chain = Src;

    type Counterparty = Dst;
}

impl RelayTarget for DestinationTarget {
    type Chain = Dst;

    type Counterparty = Src;
}

pub trait HasTargetChainTypes<Target: RelayTarget>:
    CanRaiseError<ErrorOf<Self::TargetChain>>
    + CanRaiseError<ErrorOf<Self::CounterpartyChain>>
    + HasChainTypeAt<Target::Chain, Chain = Self::TargetChain>
    + HasChainTypeAt<Target::Counterparty, Chain = Self::CounterpartyChain>
{
    type TargetChain: HasIbcChainTypes<Self::CounterpartyChain> + HasErrorType;

    type CounterpartyChain: HasIbcChainTypes<Self::TargetChain> + HasErrorType;
}

impl<Relay, Target, TargetChain, CounterpartyChain> HasTargetChainTypes<Target> for Relay
where
    Target: RelayTarget,
    Relay: CanRaiseError<TargetChain::Error>
        + CanRaiseError<CounterpartyChain::Error>
        + HasChainTypeAt<Target::Chain, Chain = TargetChain>
        + HasChainTypeAt<Target::Counterparty, Chain = CounterpartyChain>,
    TargetChain: HasIbcChainTypes<CounterpartyChain> + HasErrorType,
    CounterpartyChain: HasIbcChainTypes<TargetChain> + HasErrorType,
{
    type TargetChain = TargetChain;

    type CounterpartyChain = CounterpartyChain;
}

pub trait HasTargetChains<Target: RelayTarget>: HasTargetChainTypes<Target> {
    fn target_chain(&self) -> &Self::TargetChain;

    fn counterparty_chain(&self) -> &Self::CounterpartyChain;
}

impl<Relay, Target> HasTargetChains<Target> for Relay
where
    Target: RelayTarget,
    Relay:
        HasTargetChainTypes<Target> + HasChainAt<Target::Chain> + HasChainAt<Target::Counterparty>,
{
    fn target_chain(&self) -> &Self::TargetChain {
        self.chain_at(PhantomData::<Target::Chain>)
    }

    fn counterparty_chain(&self) -> &Self::CounterpartyChain {
        self.chain_at(PhantomData::<Target::Counterparty>)
    }
}

pub trait HasTargetClientIds<Target: RelayTarget>: HasTargetChainTypes<Target> {
    fn target_client_id(&self) -> &ClientIdOf<Self::TargetChain, Self::CounterpartyChain>;

    fn counterparty_client_id(&self) -> &ClientIdOf<Self::CounterpartyChain, Self::TargetChain>;
}

impl<Relay, Target> HasTargetClientIds<Target> for Relay
where
    Target: RelayTarget,
    Relay: HasTargetChainTypes<Target>
        + HasClientIdAt<Target::Chain, Target::Counterparty>
        + HasClientIdAt<Target::Counterparty, Target::Chain>,
{
    fn target_client_id(&self) -> &ClientIdOf<Self::TargetChain, Self::CounterpartyChain> {
        self.client_id_at(PhantomData::<(Target::Chain, Target::Counterparty)>)
    }

    fn counterparty_client_id(&self) -> &ClientIdOf<Self::CounterpartyChain, Self::TargetChain> {
        self.client_id_at(PhantomData::<(Target::Counterparty, Target::Chain)>)
    }
}

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
