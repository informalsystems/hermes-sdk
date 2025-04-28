use hermes_core::chain_type_components::traits::{AddressOf, HasAddressType};
use hermes_core::test_components::chain_driver::traits::HasChainType;
use hermes_prelude::*;

#[cgp_getter {
    provider: GovernanceProposalAuthorityGetter,
    context: Bootstrap,
}]
pub trait HasGovernanceProposalAuthority: HasChainType<Chain: HasAddressType> {
    fn governance_proposal_authority(&self) -> &AddressOf<Self::Chain>;
}
