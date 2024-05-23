use cgp_core::Async;
use hermes_test_components::chain::traits::proposal::types::proposal_id::ProvideProposalIdType;
use hermes_test_components::chain::traits::proposal::types::proposal_status::ProvideProposalStatusType;

use crate::chain::types::proposal_status::ProposalStatus;

pub struct ProvideCosmosProposalTypes;

impl<ChainDriver> ProvideProposalIdType<ChainDriver> for ProvideCosmosProposalTypes
where
    ChainDriver: Async,
{
    type ProposalId = u64;
}

impl<ChainDriver> ProvideProposalStatusType<ChainDriver> for ProvideCosmosProposalTypes
where
    ChainDriver: Async,
{
    type ProposalStatus = ProposalStatus;
}
