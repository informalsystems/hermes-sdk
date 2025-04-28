use hermes_core::relayer_components::chain::traits::{HasCommitmentProofType, HasHeightType};
use hermes_prelude::*;

#[cgp_component {
  provider: AbciQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQueryAbci: HasHeightType + HasCommitmentProofType + HasAsyncErrorType {
    async fn query_abci(
        &self,
        path: &str,
        data: &[u8],
        height: &Self::Height,
    ) -> Result<Option<Vec<u8>>, Self::Error>;

    async fn query_abci_with_proofs(
        &self,
        path: &str,
        data: &[u8],
        height: &Self::Height,
    ) -> Result<(Option<Vec<u8>>, Self::CommitmentProof), Self::Error>;
}
