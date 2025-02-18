use cgp::prelude::*;
use hermes_cosmos_chain_components::traits::message::{
    CosmosMessage, DynCosmosMessage, ToCosmosMessage,
};
use hermes_relayer_components::chain::traits::types::message::HasMessageType;
use hermes_test_components::chain::traits::proposal::messages::vote::{
    VoteProposalMessageBuilder, VoteProposalMessageBuilderComponent,
};
use hermes_test_components::chain::traits::proposal::types::proposal_id::HasProposalIdType;
use hermes_test_components::chain::traits::proposal::types::vote::HasProposalVoteType;
use ibc::primitives::Signer;
use ibc_proto::cosmos::gov::v1::MsgVote;
use ibc_proto::google::protobuf::Any;
use prost::{Message, Name};

use crate::chain::types::proposal_vote::ProposalVote;

pub struct BuildVoteProposalMessage;

#[derive(Debug)]
pub struct VoteMessage {
    pub proposal_id: u64,
    pub vote_option: i32,
}

#[cgp_provider(VoteProposalMessageBuilderComponent)]
impl<Chain> VoteProposalMessageBuilder<Chain> for BuildVoteProposalMessage
where
    Chain: HasProposalIdType<ProposalId = u64>
        + HasProposalVoteType<ProposalVote = ProposalVote>
        + HasMessageType<Message = CosmosMessage>,
{
    fn build_vote_proposal_message(
        _chain: &Chain,
        proposal_id: &u64,
        vote: &ProposalVote,
    ) -> CosmosMessage {
        let vote_option = match vote {
            ProposalVote::Yes => 1,
            ProposalVote::Abstain => 2,
            ProposalVote::No => 3,
            ProposalVote::NoWithVeto => 4,
        };

        let message = VoteMessage {
            proposal_id: *proposal_id,
            vote_option,
        };

        message.to_cosmos_message()
    }
}

impl DynCosmosMessage for VoteMessage {
    fn encode_protobuf(&self, signer: &Signer) -> Any {
        let message = MsgVote {
            proposal_id: self.proposal_id,
            voter: signer.to_string(),
            option: self.vote_option,
            metadata: String::default(),
        };

        Any {
            type_url: MsgVote::type_url(),
            value: message.encode_to_vec(),
        }
    }
}
