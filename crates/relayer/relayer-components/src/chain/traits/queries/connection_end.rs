use core::fmt::Debug;

use cgp_core::prelude::*;

use crate::chain::traits::types::connection::HasConnectionEndType;
use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::chain::traits::types::proof::HasCommitmentProofType;

#[derive_component(ConnectionEndQuerierComponent, ConnectionEndQuerier<Chain>)]
#[async_trait]
pub trait CanQueryConnectionEnd<Counterparty>:
    HasConnectionEndType<Counterparty> + HasIbcChainTypes<Counterparty> + HasErrorType
{
    async fn query_connection_end(
        &self,
        connection_id: &Self::ConnectionId,
        height: &Self::Height,
    ) -> Result<Self::ConnectionEnd, Self::Error>;
}

#[derive_component(ConnectionEndWithProofsQuerierComponent, ConnectionEndWithProofsQuerier<Chain>)]
#[async_trait]
pub trait CanQueryConnectionEndWithProofs<Counterparty>:
    HasConnectionEndType<Counterparty>
    + HasIbcChainTypes<Counterparty>
    + HasCommitmentProofType
    + HasErrorType
{
    async fn query_connection_end_with_proofs(
        &self,
        connection_id: &Self::ConnectionId,
        height: &Self::Height,
    ) -> Result<(Self::ConnectionEnd, Self::CommitmentProof), Self::Error>;
}

pub struct ConnectionNotFoundError<'a, Chain, Counterparty>
where
    Chain: HasIbcChainTypes<Counterparty>,
{
    pub chain: &'a Chain,
    pub connection_id: &'a Chain::ConnectionId,
    pub height: &'a Chain::Height,
}

impl<'a, Chain, Counterparty> Debug for ConnectionNotFoundError<'a, Chain, Counterparty>
where
    Chain: HasIbcChainTypes<Counterparty>,
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
