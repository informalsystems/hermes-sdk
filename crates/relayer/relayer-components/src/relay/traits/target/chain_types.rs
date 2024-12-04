use crate::multi::traits::chain_at::HasChainTypeAt;
use crate::relay::traits::chains::HasRelayChainTypes;
use crate::relay::traits::target::relay_target::RelayTarget;
use crate::relay::traits::target::types::{DestinationTarget, SourceTarget};

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

pub trait HasSourceTargetChainTypes:
    HasRelayChainTypes
    + HasTargetChainTypes<
        SourceTarget,
        TargetChain = Self::SrcChain,
        CounterpartyChain = Self::DstChain,
    >
{
}

impl<Relay> HasSourceTargetChainTypes for Relay where Relay: HasRelayChainTypes {}

pub trait HasDestinationTargetChainTypes:
    HasRelayChainTypes
    + HasTargetChainTypes<
        DestinationTarget,
        TargetChain = Self::DstChain,
        CounterpartyChain = Self::SrcChain,
    >
{
}

impl<Relay> HasDestinationTargetChainTypes for Relay where Relay: HasRelayChainTypes {}

pub type TargetChainOf<Relay, Target> = <Relay as HasTargetChainTypes<Target>>::TargetChain;

pub type CounterpartyChainOf<Relay, Target> =
    <Relay as HasTargetChainTypes<Target>>::CounterpartyChain;
