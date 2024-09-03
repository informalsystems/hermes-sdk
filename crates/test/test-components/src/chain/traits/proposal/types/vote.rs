use cgp::prelude::*;

#[derive_component(ProposalVoteTypeComponent, ProvideProposalVoteType<Chain>)]
pub trait HasProposalVoteType: Async {
    type ProposalVote: Async;
}
