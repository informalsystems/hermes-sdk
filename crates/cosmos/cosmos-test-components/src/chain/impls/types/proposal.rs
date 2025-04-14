use cgp::prelude::*;
use hermes_test_components::chain::traits::{
    ProposalIdTypeComponent, ProposalStatusTypeComponent, ProposalVoteTypeComponent,
    ProvideProposalIdType, ProvideProposalStatusType, ProvideProposalVoteType,
};

use crate::chain::types::proposal_status::ProposalStatus;
use crate::chain::types::proposal_vote::ProposalVote;

pub struct ProvideCosmosProposalTypes;

#[cgp_provider(ProposalIdTypeComponent)]
impl<Chain: Async> ProvideProposalIdType<Chain> for ProvideCosmosProposalTypes {
    type ProposalId = u64;
}

#[cgp_provider(ProposalStatusTypeComponent)]
impl<Chain: Async> ProvideProposalStatusType<Chain> for ProvideCosmosProposalTypes {
    type ProposalStatus = ProposalStatus;
}

#[cgp_provider(ProposalVoteTypeComponent)]
impl<Chain: Async> ProvideProposalVoteType<Chain> for ProvideCosmosProposalTypes {
    type ProposalVote = ProposalVote;
}
