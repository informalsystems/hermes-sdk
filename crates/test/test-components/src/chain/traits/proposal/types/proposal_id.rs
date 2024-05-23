use cgp_core::prelude::*;

#[derive_component(ProposalIdTypeComponent, ProvideProposalIdType<Chain>)]
pub trait HasProposalIdType: Async {
    type ProposalId: Async;
}

pub type ProposalIdOf<Chain> = <Chain as HasProposalIdType>::ProposalId;
