use core::marker::PhantomData;

use hermes_chain_components::traits::HasClientIdType;
use hermes_chain_components::types::aliases::ClientIdOf;

use crate::multi::traits::client_id_at::HasClientIdAt;
use crate::relay::traits::{HasTargetChainTypes, RelayTarget};

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
