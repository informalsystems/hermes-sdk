use std::sync::Arc;

use cgp::prelude::*;
use hermes_chain_type_components::traits::{HasEventType, HasHeightType};
use hermes_relayer_components::chain::traits::{BlockEventsQuerier, BlockEventsQuerierComponent};
use ibc::core::client::types::Height;
use tendermint::abci::Code;
use tendermint::block::Height as TmHeight;
use tendermint::Error as TmError;
use tendermint_rpc::{Client, Error as RpcError};

use crate::traits::HasRpcClient;
use crate::types::CosmosEvent;

pub struct QueryCosmosBlockEvents;

#[cgp_provider(BlockEventsQuerierComponent)]
impl<Chain> BlockEventsQuerier<Chain> for QueryCosmosBlockEvents
where
    Chain: HasHeightType<Height = Height>
        + HasEventType<Event = CosmosEvent>
        + HasRpcClient
        + CanRaiseAsyncError<TmError>
        + CanRaiseAsyncError<RpcError>,
{
    async fn query_block_events(
        chain: &Chain,
        height: &Chain::Height,
    ) -> Result<Vec<Chain::Event>, Chain::Error> {
        let tendermint_height =
            TmHeight::try_from(height.revision_height()).map_err(Chain::raise_error)?;

        let response = chain
            .rpc_client()
            .block_results(tendermint_height)
            .await
            .map_err(Chain::raise_error)?;

        let mut events = Vec::new();

        if let Some(begin_block_events) = response.begin_block_events {
            events.extend(begin_block_events.into_iter().map(Arc::new));
        }

        if let Some(txs_results) = response.txs_results {
            for tx_result in txs_results {
                if tx_result.code == Code::Ok {
                    events.extend(tx_result.events.into_iter().map(Arc::new));
                }
            }
        }

        if let Some(end_block_events) = response.end_block_events {
            events.extend(end_block_events.into_iter().map(Arc::new));
        }

        Ok(events)
    }
}
