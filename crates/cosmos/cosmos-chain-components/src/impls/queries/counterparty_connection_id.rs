use cgp::prelude::*;
use hermes_relayer_components::chain::traits::{
    CanQueryChainHeight, CanQueryConnectionEnd, CounterpartyConnectionIdQuerier,
    CounterpartyConnectionIdQuerierComponent, HasChannelEndType, HasConnectionIdType,
};
use ibc::core::channel::types::channel::ChannelEnd;
use ibc::core::connection::types::ConnectionEnd;
use ibc::core::host::types::identifiers::ConnectionId;

#[cgp_new_provider(CounterpartyConnectionIdQuerierComponent)]
impl<Chain, Counterparty> CounterpartyConnectionIdQuerier<Chain, Counterparty>
    for QueryCounterpartyConnectionId
where
    Chain: CanQueryChainHeight
        + HasConnectionIdType<Counterparty>
        + HasChannelEndType<Counterparty, ChannelEnd = ChannelEnd>
        + CanQueryConnectionEnd<
            Counterparty,
            ConnectionEnd = ConnectionEnd,
            ConnectionId = ConnectionId,
        > + CanRaiseAsyncError<String>,
    Counterparty: HasConnectionIdType<Chain, ConnectionId = ConnectionId>,
{
    async fn query_channel_end_counterparty_connection_id(
        chain: &Chain,
        channel_end: &ChannelEnd,
    ) -> Result<ConnectionId, Chain::Error> {
        let [connection_id]: [ConnectionId; 1] = channel_end
            .connection_hops
            .clone()
            .try_into()
            .map_err(|_| {
                Chain::raise_error(
                    "channel end must have exactly one connection ID in connection_hops"
                        .to_string(),
                )
            })?;

        let latest_height = chain.query_chain_height().await?;

        let connection_end = chain
            .query_connection_end(&connection_id, &latest_height)
            .await?;

        let counterparty_connection_id = connection_end
            .counterparty()
            .connection_id
            .clone()
            .ok_or_else(|| {
                Chain::raise_error(format!(
                    "connection end with id `{connection_id}` has no counterparty connection"
                ))
            })?;

        Ok(counterparty_connection_id)
    }
}
