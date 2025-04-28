use hermes_prelude::*;
use hermes_relayer_components::chain::traits::HasMessageType;

use crate::chain::traits::{HasProposalIdType, HasProposalVoteType};

#[cgp_component {
  provider: VoteProposalMessageBuilder,
  context: Chain,
}]
pub trait CanBuildVoteProposalMessage:
    HasProposalIdType + HasProposalVoteType + HasMessageType
{
    fn build_vote_proposal_message(
        &self,
        proposal_id: &Self::ProposalId,
        vote: &Self::ProposalVote,
    ) -> Self::Message;
}
