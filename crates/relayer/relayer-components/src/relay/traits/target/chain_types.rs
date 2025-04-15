use cgp::core::macros::blanket_trait;

use crate::multi::traits::chain_at::HasChainTypeAt;
use crate::relay::traits::{DestinationTarget, HasRelayChainTypes, RelayTarget, SourceTarget};

#[blanket_trait]
pub trait HasTargetChainTypes<Target: RelayTarget>:
    HasChainTypeAt<Target::Chain, Chain = Self::TargetChain>
    + HasChainTypeAt<Target::Counterparty, Chain = Self::CounterpartyChain>
{
    type TargetChain;

    type CounterpartyChain;
}

#[blanket_trait]
pub trait HasSourceTargetChainTypes:
    HasRelayChainTypes
    + HasTargetChainTypes<
        SourceTarget,
        TargetChain = Self::SrcChain,
        CounterpartyChain = Self::DstChain,
    >
{
}

#[blanket_trait]
pub trait HasDestinationTargetChainTypes:
    HasRelayChainTypes
    + HasTargetChainTypes<
        DestinationTarget,
        TargetChain = Self::DstChain,
        CounterpartyChain = Self::SrcChain,
    >
{
}

#[blanket_trait]
pub trait HasChainTargets: HasSourceTargetChainTypes + HasDestinationTargetChainTypes {}

pub type TargetChainOf<Relay, Target> = <Relay as HasTargetChainTypes<Target>>::TargetChain;

pub type CounterpartyChainOf<Relay, Target> =
    <Relay as HasTargetChainTypes<Target>>::CounterpartyChain;
