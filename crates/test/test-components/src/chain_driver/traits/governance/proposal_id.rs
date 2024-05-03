use cgp_core::prelude::*;

#[derive_component(ProposalIdTypeComponent, ProvideProposalIdType<ChainDriver>)]
pub trait HasProposalIdType: Async {
    type ProposalId: Async;
}
