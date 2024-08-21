use cgp_core::Async;
use hermes_test_components::chain::traits::proposal::types::proposal_id::ProvideProposalIdType;
use hermes_test_components::chain::traits::proposal::types::proposal_status::ProvideProposalStatusType;
use hermes_test_components::chain::traits::proposal::types::vote::ProvideProposalVoteType;

use crate::chain::types::proposal_status::ProposalStatus;
use crate::chain::types::proposal_vote::ProposalVote;

pub struct ProvideCosmosProposalTypes;

impl<Chain: Async> ProvideProposalIdType<Chain> for ProvideCosmosProposalTypes {
    type ProposalId = u64;
}

impl<Chain: Async> ProvideProposalStatusType<Chain> for ProvideCosmosProposalTypes {
    type ProposalStatus = ProposalStatus;
}

impl<Chain: Async> ProvideProposalVoteType<Chain> for ProvideCosmosProposalTypes {
    type ProposalVote = ProposalVote;
}
