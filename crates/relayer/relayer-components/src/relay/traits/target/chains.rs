use core::marker::PhantomData;

use crate::multi::traits::chain_at::HasChainAt;
use crate::relay::traits::{HasTargetChainTypes, RelayTarget};

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
