use hermes_prelude::*;

use crate::traits::types::commitment::path::HasCommitmentPathType;
use crate::traits::types::commitment::value::HasCommitmentValueType;

#[cgp_component {
  provider: CommitmentStorage,
  context: Chain,
}]
#[async_trait]
pub trait CanStoreCommitment<Tag>:
    HasCommitmentPathType<Tag> + HasCommitmentValueType<Tag> + HasAsyncErrorType
{
    async fn store_commitment(
        &mut self,
        path: &Self::CommitmentPath,
        value: &Self::CommitmentValue,
    ) -> Result<(), Self::Error>;
}
