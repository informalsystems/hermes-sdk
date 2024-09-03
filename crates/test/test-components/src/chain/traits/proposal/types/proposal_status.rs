use cgp::prelude::*;

#[derive_component(ProposalStatusTypeComponent, ProvideProposalStatusType<ChainDriver>)]
pub trait HasProposalStatusType: Async {
    type ProposalStatus: Async;
}

pub type ProposalStatusOf<Chain> = <Chain as HasProposalStatusType>::ProposalStatus;
