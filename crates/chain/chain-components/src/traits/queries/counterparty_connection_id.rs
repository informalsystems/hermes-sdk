use hermes_chain_type_components::traits::HasConnectionIdType;
use hermes_prelude::*;

use crate::traits::HasChannelEndType;

#[cgp_component {
    provider: CounterpartyConnectionIdQuerier,
    context: Chain,
}]
#[async_trait]
pub trait CanQueryCounterpartyConnectionId<Counterparty>:
    HasChannelEndType<Counterparty> + HasAsyncErrorType + Sized
where
    Counterparty: HasConnectionIdType<Self>,
{
    async fn query_channel_end_counterparty_connection_id(
        &self,
        channel_end: &Self::ChannelEnd,
    ) -> Result<Counterparty::ConnectionId, Self::Error>;
}
