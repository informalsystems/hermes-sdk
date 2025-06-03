use core::fmt::Debug;

use hermes_prelude::*;

#[cgp_component {
  name: ProposalStatusTypeComponent,
  provider: ProvideProposalStatusType,
  context: ChainDriver,
}]
pub trait HasProposalStatusType: Async {
    type ProposalStatus: Async + Debug;
}

pub type ProposalStatusOf<Chain> = <Chain as HasProposalStatusType>::ProposalStatus;
