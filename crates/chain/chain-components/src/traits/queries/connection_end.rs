use core::fmt::Debug;

use cgp::prelude::*;
use hermes_chain_type_components::traits::{HasConnectionIdType, HasHeightType};

use crate::traits::{HasCommitmentProofType, HasConnectionEndType};

#[cgp_component {
  provider: ConnectionEndQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQueryConnectionEnd<Counterparty>:
    HasHeightType
    + HasConnectionIdType<Counterparty>
    + HasConnectionEndType<Counterparty>
    + HasAsyncErrorType
{
    async fn query_connection_end(
        &self,
        connection_id: &Self::ConnectionId,
        height: &Self::Height,
    ) -> Result<Self::ConnectionEnd, Self::Error>;
}

#[cgp_component {
  provider: ConnectionEndWithProofsQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQueryConnectionEndWithProofs<Counterparty>:
    HasHeightType
    + HasConnectionIdType<Counterparty>
    + HasConnectionEndType<Counterparty>
    + HasCommitmentProofType
    + HasAsyncErrorType
{
    async fn query_connection_end_with_proofs(
        &self,
        connection_id: &Self::ConnectionId,
        height: &Self::Height,
    ) -> Result<(Self::ConnectionEnd, Self::CommitmentProof), Self::Error>;
}

pub struct ConnectionNotFoundError<'a, Chain, Counterparty>
where
    Chain: HasHeightType + HasConnectionIdType<Counterparty>,
{
    pub chain: &'a Chain,
    pub connection_id: &'a Chain::ConnectionId,
    pub height: &'a Chain::Height,
}

impl<Chain, Counterparty> Debug for ConnectionNotFoundError<'_, Chain, Counterparty>
where
    Chain: HasHeightType + HasConnectionIdType<Counterparty>,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(
            f,
            "connection not found with connection id {} at height {}",
            self.connection_id, self.height
        )?;
        Ok(())
    }
}
