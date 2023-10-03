use cgp_async::async_trait;
use cgp_core::traits::HasErrorType;

use crate::chain::traits::types::client_state::HasClientStateType;
use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::std_prelude::*;

#[async_trait]
pub trait CanQueryClientState<Counterparty>: HasIbcChainTypes<Counterparty> + HasErrorType
where
    Counterparty: HasClientStateType<Self>,
{
    async fn query_client_state(
        &self,
        client_id: &Self::ClientId,
    ) -> Result<Counterparty::ClientState, Self::Error>;
}
