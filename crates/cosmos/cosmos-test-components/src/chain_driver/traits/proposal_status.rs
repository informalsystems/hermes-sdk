use cgp_core::prelude::*;
use hermes_runtime_components::traits::fs::file_path::HasFilePathType;
use hermes_runtime_components::traits::runtime::HasRuntimeType;
use hermes_test_components::chain_driver::traits::governance::proposal_id::HasProposalIdType;

#[derive_component(GovernanceProposalStatusQuerierComponent, GovernanceProposalStatusQuerier<ChainDriver>)]
#[async_trait]
pub trait CanQueryGovernanceProposalStatus:
    HasProposalIdType + HasRuntimeType + HasErrorType
where
    Self::Runtime: HasFilePathType,
{
    async fn query_proposal_status(
        &self,
        proposal_id: &Self::ProposalId,
    ) -> Result<String, Self::Error>;
}
