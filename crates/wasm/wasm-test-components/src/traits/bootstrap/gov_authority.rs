use core::marker::PhantomData;

use cgp::core::component::UseContext;
use cgp::prelude::*;
use hermes_test_components::chain::traits::types::address::{AddressOf, HasAddressType};
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;

#[cgp_component {
  provider: GovernanceProposalAuthorityGetter,
  context: Bootstrap,
}]
pub trait HasGovernanceProposalAuthority: HasChainType<Chain: HasAddressType> {
    fn governance_proposal_authority(&self) -> &AddressOf<Self::Chain>;
}

#[cgp_provider(GovernanceProposalAuthorityGetterComponent)]
impl<Bootstrap, Chain> GovernanceProposalAuthorityGetter<Bootstrap> for UseContext
where
    Bootstrap: HasChainType<Chain = Chain>
        + HasField<symbol!("governance_proposal_authority"), Value = Chain::Address>,
    Chain: HasAddressType,
{
    fn governance_proposal_authority(bootstrap: &Bootstrap) -> &Chain::Address {
        bootstrap.get_field(PhantomData)
    }
}
