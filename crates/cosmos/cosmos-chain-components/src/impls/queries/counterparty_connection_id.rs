use cgp::prelude::CanRaiseAsyncError;
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainHeight;
use hermes_relayer_components::chain::traits::queries::connection_end::CanQueryConnectionEnd;
use hermes_relayer_components::chain::traits::queries::counterparty_connection_id::CounterpartyConnectionIdQuerier;
use hermes_relayer_components::chain::traits::types::channel::HasChannelEndType;
use hermes_relayer_components::chain::traits::types::ibc::HasConnectionIdType;
use ibc::core::channel::types::channel::ChannelEnd;
use ibc::core::connection::types::ConnectionEnd;
use ibc::core::host::types::identifiers::ConnectionId;

pub struct QueryCounterpartyConnectionId;

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
        channel_end: &Chain::ChannelEnd,
    ) -> Result<Counterparty::ConnectionId, Chain::Error> {
        let connection_id = channel_end
            .connection_hops
            .first()
            .ok_or_else(|| Chain::raise_error(format!("channel end has no connection_hops")))?;

        let latest_height = chain.query_chain_height().await?;

        let connection_end = chain
            .query_connection_end(connection_id, &latest_height)
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
