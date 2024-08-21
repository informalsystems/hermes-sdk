use cgp_core::prelude::*;
use hermes_test_components::chain::traits::types::address::{AddressOf, HasAddressType};
use hermes_test_components::chain_driver::traits::types::chain::HasChainType;

#[derive_component(GovernanceProposalAuthorityGetterComponent, GovernanceProposalAuthorityGetter<Bootstrap>)]
pub trait HasGovernanceProposalAuthority: HasChainType<Chain: HasAddressType> {
    fn governance_proposal_authority(&self) -> &AddressOf<Self::Chain>;
}
