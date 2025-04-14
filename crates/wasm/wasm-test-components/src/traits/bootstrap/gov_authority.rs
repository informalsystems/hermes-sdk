use cgp::prelude::*;
use hermes_chain_type_components::traits::{AddressOf, HasAddressType};
use hermes_test_components::chain_driver::traits::HasChainType;

#[cgp_getter {
    provider: GovernanceProposalAuthorityGetter,
    context: Bootstrap,
}]
pub trait HasGovernanceProposalAuthority: HasChainType<Chain: HasAddressType> {
    fn governance_proposal_authority(&self) -> &AddressOf<Self::Chain>;
}
