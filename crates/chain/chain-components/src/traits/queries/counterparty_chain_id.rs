use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::channel_id::HasChannelIdType;
use hermes_chain_type_components::traits::types::ibc::port_id::HasPortIdType;

use crate::traits::types::chain_id::HasChainIdType;

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
