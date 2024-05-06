use cgp_core::prelude::*;
use hermes_test_components::chain::traits::proposal::types::proposal_id::HasProposalIdType;
use hermes_test_components::chain::traits::proposal::types::proposal_status::HasProposalStatusType;

#[derive_component(GovernanceProposalStatusQuerierComponent, GovernanceProposalStatusQuerier<ChainDriver>)]
#[async_trait]
pub trait CanQueryGovernanceProposalStatus:
    HasProposalIdType + HasProposalStatusType + HasErrorType
{
    async fn query_proposal_status(
        &self,
        proposal_id: &Self::ProposalId,
    ) -> Result<Self::ProposalStatus, Self::Error>;
}

#[derive_component(GovernanceProposalStatusPollerComponent, GovernanceProposalStatusPoller<ChainDriver>)]
#[async_trait]
pub trait CanPollProposalStatus: HasProposalIdType + HasProposalStatusType + HasErrorType {
    async fn poll_proposal_status(
        &self,
        proposal_id: &Self::ProposalId,
        expected_status: &Self::ProposalStatus,
    ) -> Result<(), Self::Error>;
}
