use oneline_eyre::eyre::Context;
use tracing::{info, warn};

use hermes_cli_framework::command::Runnable;
use hermes_cli_framework::output::Output;
use hermes_cosmos_client_components::traits::chain_handle::HasBlockingChainHandle;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
use hermes_cosmos_relayer::types::error::BaseError;
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainHeight;

use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer::chain::requests::{
    IncludeProof, PageRequest, QueryChannelsRequest, QueryClientStateRequest,
    QueryConnectionRequest, QueryHeight,
};
use ibc_relayer_types::core::ics04_channel::channel::State;
use ibc_relayer_types::core::ics24_host::identifier::{ChainId, PortChannelId};

use crate::Result;

#[derive(Debug, clap::Parser)]
pub struct QueryChannels {
    #[clap(
        long = "chain",
        required = true,
        value_name = "CHAIN_ID",
        help_heading = "REQUIRED",
        help = "Identifier of the chain to query"
    )]
    chain_id: ChainId,

    #[clap(
        long = "counterparty-chain",
        value_name = "COUNTERPARTY_CHAIN_ID",
        help = "Filter the query response by the counterparty chain"
    )]
    counterparty_chain_id: Option<ChainId>,

    #[clap(
        long = "show-counterparty",
        help = "Show the counterparty's chain, port, and channel"
    )]
    show_counterparty: bool,
}

impl Runnable for QueryChannels {
    async fn run(&self, builder: CosmosBuilder) -> Result<Output> {
        let chain = builder.build_chain(&self.chain_id).await?;
        let chain_id = self.chain_id.clone();
        let dst_chain_id = self.counterparty_chain_id.clone();
        let show_counterparty = self.show_counterparty;

        let all_channels = chain
            .with_blocking_chain_handle(move |chain_handle| {
                chain_handle
                    .query_channels(QueryChannelsRequest {
                        pagination: Some(PageRequest::all()),
                    })
                    .map_err(|e| BaseError::relayer(e).into())
            })
            .await?;

        let chain_height = chain
            .query_chain_height()
            .await
            .wrap_err("Failed to query latest chain height")?;

        let mut channels = Vec::new();

        for channel in all_channels {
            let port_id = &channel.port_id;
            let channel_id = &channel.channel_id;
            let chain_id = chain_id.clone();
            let channel_end = &channel.channel_end;

            if channel_end.state_matches(&State::Uninitialized) {
                warn!("{port_id}/{channel_id} on chain {chain_id} at {chain_height:?} is uninitialized");

                continue;
            }

            let Some(connection_id) = channel.channel_end.connection_hops.first() else {
                warn!(
                    "missing connection hops for {}/{} on chain {} @ {:?}",
                    port_id, channel_id, chain_id, chain_height,
                );

                continue;
            };

            if show_counterparty || dst_chain_id.is_some() {
                let connection_id = connection_id.clone();
                let connection_end = chain
                    .with_blocking_chain_handle(move |handle| {
                        handle
                            .query_connection(
                                QueryConnectionRequest {
                                    connection_id,
                                    height: QueryHeight::Specific(chain_height),
                                },
                                IncludeProof::No,
                            )
                            .map_err(|e| BaseError::relayer(e).into())
                    })
                    .await;

                let Ok((connection_end, _)) = connection_end else {
                    warn!(
                        "missing connection end for {}/{} on chain {} @ {:?}",
                        port_id, channel_id, chain_id, chain_height,
                    );

                    continue;
                };

                let client_id = connection_end.client_id().clone();
                let client_state = chain
                    .with_blocking_chain_handle(move |handle| {
                        handle
                            .query_client_state(
                                QueryClientStateRequest {
                                    client_id,
                                    height: QueryHeight::Specific(chain_height),
                                },
                                IncludeProof::No,
                            )
                            .map_err(|e| BaseError::relayer(e).into())
                    })
                    .await;

                let Ok((client_state, _)) = client_state else {
                    warn!("missing client state for {port_id}/{channel_id} on chain {chain_id} at {chain_height:?}");

                    continue;
                };

                let client_state_chain_id = client_state.chain_id();
                let client_state_chain_id_matches_dst_chain_id = dst_chain_id
                    .as_ref()
                    .map(|dst_chain_id| dst_chain_id == &client_state_chain_id)
                    .unwrap_or(true);

                if !client_state_chain_id_matches_dst_chain_id {
                    continue;
                }

                channels.push(channel);
            }
        }

        info!("Successfully queried channels on chain `{chain_id}`");

        channels.iter().for_each(|channel| {
            let port_id = &channel.port_id;
            let channel_id = &channel.channel_id;

            info!(
                "{:?}",
                PortChannelId {
                    channel_id: channel_id.clone(),
                    port_id: port_id.clone(),
                }
            );
        });

        Ok(Output::success(channels))
    }
}
