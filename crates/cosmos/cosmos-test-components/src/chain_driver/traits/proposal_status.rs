use cgp_core::prelude::*;
use hermes_runtime_components::traits::fs::file_path::HasFilePathType;
use hermes_runtime_components::traits::runtime::HasRuntimeType;

#[derive_component(GovernanceProposalStatusQuerierComponent, GovernanceProposalStatusQuerier<ChainDriver>)]
#[async_trait]
pub trait CanQueryGovernanceProposalStatus: HasRuntimeType + HasErrorType
where
    Self::Runtime: HasFilePathType,
{
    async fn query_proposal_status(&self, proposal_id: &str) -> Result<String, Self::Error>;
}
