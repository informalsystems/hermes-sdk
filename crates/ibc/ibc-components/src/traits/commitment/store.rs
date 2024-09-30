use cgp::prelude::*;

use crate::traits::types::commitment::path::HasCommitmentPathType;
use crate::traits::types::commitment::value::HasCommitmentValueType;

#[async_trait]
pub trait CanStoreCommitment:
    HasCommitmentPathType + HasCommitmentValueType + HasErrorType
{
    async fn store_commitment(
        &self,
        path: &Self::CommitmentPath,
        value: &Self::CommitmentValue,
    ) -> Result<(), Self::Error>;
}
