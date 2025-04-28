use hermes_chain_type_components::traits::{HasChannelIdType, HasPortIdType};
use hermes_prelude::*;

use crate::traits::HasChainIdType;

#[cgp_component {
  provider: CounterpartyChainIdQuerier,
  context: Chain,
}]
#[async_trait]
pub trait CanQueryCounterpartyChainId<Counterparty>:
    HasChannelIdType<Counterparty> + HasPortIdType<Counterparty> + HasAsyncErrorType
where
    Counterparty: HasChainIdType,
{
    async fn query_counterparty_chain_id_from_channel_id(
        &self,
        channel_id: &Self::ChannelId,
        port_id: &Self::PortId,
    ) -> Result<Counterparty::ChainId, Self::Error>;
}
