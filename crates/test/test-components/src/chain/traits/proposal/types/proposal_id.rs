use hermes_prelude::*;

#[cgp_component {
  name: ProposalIdTypeComponent,
  provider: ProvideProposalIdType,
  context: Chain,
}]
pub trait HasProposalIdType: Async {
    type ProposalId: Async;
}

pub type ProposalIdOf<Chain> = <Chain as HasProposalIdType>::ProposalId;
