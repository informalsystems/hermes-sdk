use async_trait::async_trait;

use crate::chain::traits::types::chain::HasChainTypes;
use crate::chain::traits::types::ibc::HasIbcChainTypes;
use crate::std_prelude::*;
use cgp_core::traits::error::HasErrorType;

#[async_trait]
pub trait CanQueryCounterpartyChainIdFromChannel<Counterparty>:
    HasIbcChainTypes<Counterparty> + HasErrorType
where
    Counterparty: HasChainTypes,
{
    async fn query_chain_id_from_channel_id(
        &self,
        channel_id: &Self::ChannelId,
        port_id: &Self::PortId,
    ) -> Result<Counterparty::ChainId, Self::Error>;
}
