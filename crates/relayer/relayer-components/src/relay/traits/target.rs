use core::marker::PhantomData;

use cgp::core::Async;
use hermes_chain_components::traits::types::ibc::HasClientIdType;

use crate::chain::types::aliases::ClientIdOf;
use crate::multi::traits::chain_at::{HasChainAt, HasChainTypeAt};
use crate::multi::traits::client_id_at::HasClientIdAt;
use crate::multi::types::tags::{Dst, Src};
use crate::relay::traits::chains::HasRelayChainTypes;

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
    HasChainTypeAt<Target::Chain, Chain = Self::TargetChain>
    + HasChainTypeAt<Target::Counterparty, Chain = Self::CounterpartyChain>
{
    type TargetChain;

    type CounterpartyChain;
}

impl<Relay, Target, TargetChain, CounterpartyChain> HasTargetChainTypes<Target> for Relay
where
    Target: RelayTarget,
    Relay: HasChainTypeAt<Target::Chain, Chain = TargetChain>
        + HasChainTypeAt<Target::Counterparty, Chain = CounterpartyChain>,
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

pub trait HasTargetClientIds<Target: RelayTarget>:
    HasTargetChainTypes<
    Target,
    TargetChain: HasClientIdType<Self::CounterpartyChain>,
    CounterpartyChain: HasClientIdType<Self::TargetChain>,
>
{
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

pub trait HasSourceTargetChainTypes:
    HasRelayChainTypes
    + HasTargetChainTypes<
        SourceTarget,
        TargetChain = Self::SrcChain,
        CounterpartyChain = Self::DstChain,
    >
{
}

impl<Relay> HasSourceTargetChainTypes for Relay where
    Relay: HasRelayChainTypes
        + HasTargetChainTypes<
            SourceTarget,
            TargetChain = Self::SrcChain,
            CounterpartyChain = Self::DstChain,
        >
{
}

pub trait HasDestinationTargetChainTypes:
    HasRelayChainTypes
    + HasTargetChainTypes<
        DestinationTarget,
        TargetChain = Self::DstChain,
        CounterpartyChain = Self::SrcChain,
    >
{
}

impl<Relay> HasDestinationTargetChainTypes for Relay where
    Relay: HasRelayChainTypes
        + HasTargetChainTypes<
            DestinationTarget,
            TargetChain = Self::DstChain,
            CounterpartyChain = Self::SrcChain,
        >
{
}

pub type TargetChainOf<Relay, Target> = <Relay as HasTargetChainTypes<Target>>::TargetChain;

pub type CounterpartyChainOf<Relay, Target> =
    <Relay as HasTargetChainTypes<Target>>::CounterpartyChain;

impl private::Sealed for SourceTarget {}
impl private::Sealed for DestinationTarget {}

mod private {
    pub trait Sealed {}
}
