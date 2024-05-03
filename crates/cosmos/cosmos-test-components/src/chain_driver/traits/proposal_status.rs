use cgp_core::prelude::*;
use hermes_test_components::chain_driver::traits::governance::proposal_id::HasProposalIdType;

#[derive_component(GovernanceProposalStatusQuerierComponent, GovernanceProposalStatusQuerier<ChainDriver>)]
#[async_trait]
pub trait CanQueryGovernanceProposalStatus: HasProposalIdType + HasErrorType {
    async fn query_proposal_status(
        &self,
        proposal_id: &Self::ProposalId,
    ) -> Result<String, Self::Error>;
}

#[derive_component(GovernanceProposalStatusPollerComponent, GovernanceProposalStatusPoller<ChainDriver>)]
#[async_trait]
pub trait CanPollProposalStatus: HasProposalIdType + HasErrorType {
    async fn poll_proposal_status(
        &self,
        proposal_id: &Self::ProposalId,
        expected_status: &str,
    ) -> Result<(), Self::Error>;
}
