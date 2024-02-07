use cgp_core::prelude::*;
use hermes_relayer_components::runtime::traits::runtime::HasRuntimeType;
use hermes_test_components::runtime::traits::types::file_path::HasFilePathType;

#[derive_component(GovernanceProposalStatusQuerierComponent, GovernanceProposalStatusQuerier<ChainDriver>)]
#[async_trait]
pub trait CanQueryGovernanceProposalStatus: HasRuntimeType + HasErrorType
where
    Self::Runtime: HasFilePathType,
{
    async fn query_proposal_status(&self, proposal_id: &str) -> Result<String, Self::Error>;
}
