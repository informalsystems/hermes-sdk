use cgp::prelude::*;
use hermes_chain_type_components::traits::types::ibc::connection_id::HasConnectionIdType;

use crate::traits::types::channel::HasChannelEndType;

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
