use cgp::prelude::*;
use hermes_test_components::chain::traits::types::address::{AddressOf, HasAddressType};
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;

#[cgp_getter {
    provider: GovernanceProposalAuthorityGetter,
    context: Bootstrap,
}]
pub trait HasGovernanceProposalAuthority: HasChainType<Chain: HasAddressType> {
    fn governance_proposal_authority(&self) -> &AddressOf<Self::Chain>;
}
