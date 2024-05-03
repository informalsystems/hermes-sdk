use cgp_core::Async;
use hermes_test_components::chain_driver::traits::governance::proposal_id::ProvideProposalIdType;

pub struct ProvideU64ProposalId;

impl<ChainDriver> ProvideProposalIdType<ChainDriver> for ProvideU64ProposalId
where
    ChainDriver: Async,
{
    type ProposalId = u64;
}
