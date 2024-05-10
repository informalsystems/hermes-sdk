use cgp_core::prelude::*;

use crate::chain::traits::types::connection::HasConnectionEndType;
use crate::chain::traits::types::ibc::HasIbcChainTypes;

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
