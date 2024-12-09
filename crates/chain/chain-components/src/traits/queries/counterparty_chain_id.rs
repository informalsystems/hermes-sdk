use cgp::prelude::*;

use crate::traits::types::chain_id::HasChainIdType;
use crate::traits::types::ibc::HasIbcChainTypes;

#[cgp_component {
  provider: CounterpartyChainIdQuerier,
  context: Chain,
}]
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
