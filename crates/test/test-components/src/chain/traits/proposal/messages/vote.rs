use cgp_core::prelude::*;
use hermes_relayer_components::chain::traits::types::message::HasMessageType;

use crate::chain::traits::proposal::types::proposal_id::HasProposalIdType;
use crate::chain::traits::proposal::types::vote::HasProposalVoteType;

#[derive_component(VoteProposalMessageBuilderComponent, VoteProposalMessageBuilder<Chain>)]
pub trait CanBuildVoteProposalMessage:
    HasProposalIdType + HasProposalVoteType + HasMessageType
{
    fn build_vote_proposal_message(
        &self,
        proposal_id: &Self::ProposalId,
        vote: &Self::ProposalVote,
    ) -> Self::Message;
}
