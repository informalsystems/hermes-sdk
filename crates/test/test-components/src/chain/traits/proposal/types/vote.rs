use hermes_prelude::*;

#[cgp_component {
  name: ProposalVoteTypeComponent,
  provider: ProvideProposalVoteType,
  context: Chain,
}]
pub trait HasProposalVoteType: Async {
    type ProposalVote: Async;
}
