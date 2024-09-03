use cgp::prelude::*;

use crate::chain::traits::types::chain_id::HasChainIdType;
use crate::chain::traits::types::ibc::HasIbcChainTypes;

#[derive_component(CounterpartyChainIdQuerierComponent, CounterpartyChainIdQuerier<Chain>)]
#[async_trait]
pub trait CanQueryCounterpartyChainId<Counterparty>:
    HasIbcChainTypes<Counterparty> + HasErrorType
where
    Counterparty: HasChainIdType,
{
    async fn query_counterparty_chain_id_from_channel_id(
        &self,
        channel_id: &Self::ChannelId,
        port_id: &Self::PortId,
    ) -> Result<Counterparty::ChainId, Self::Error>;
}
