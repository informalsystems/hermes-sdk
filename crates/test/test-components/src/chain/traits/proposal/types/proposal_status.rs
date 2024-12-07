use cgp::prelude::*;

#[cgp_component {
  name: ProposalStatusTypeComponent,
  provider: ProvideProposalStatusType,
  context: ChainDriver,
}]
pub trait HasProposalStatusType: Async {
    type ProposalStatus: Async;
}

pub type ProposalStatusOf<Chain> = <Chain as HasProposalStatusType>::ProposalStatus;
