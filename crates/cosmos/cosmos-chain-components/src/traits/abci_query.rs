use cgp::prelude::*;
use hermes_relayer_components::chain::traits::types::height::HasHeightType;
use hermes_relayer_components::chain::traits::types::proof::HasCommitmentProofType;

#[cgp_component {
  provider: AbciQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQueryAbci: HasHeightType + HasCommitmentProofType + HasErrorType {
    async fn query_abci(
        &self,
        path: &str,
        data: &[u8],
        height: &Self::Height,
    ) -> Result<Vec<u8>, Self::Error>;

    async fn query_abci_with_proofs(
        &self,
        path: &str,
        data: &[u8],
        height: &Self::Height,
    ) -> Result<(Vec<u8>, Self::CommitmentProof), Self::Error>;
}
