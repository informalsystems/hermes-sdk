use cgp::prelude::*;
use hermes_chain_type_components::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::proof::HasCommitmentProofType;
use ibc_proto::cosmos::staking::v1beta1::Params;

#[derive_component(StakingParamsQuerierComponent, StakingParamsQuerier<Chain>)]
#[async_trait]
pub trait CanQueryStakingParams: HasHeightType + HasCommitmentProofType + HasErrorType {
    async fn query_staking_params(&self, height: &Self::Height) -> Result<Params, Self::Error>;

    async fn query_staking_params_with_proofs(
        &self,
        height: &Self::Height,
    ) -> Result<(Params, Self::CommitmentProof), Self::Error>;
}
