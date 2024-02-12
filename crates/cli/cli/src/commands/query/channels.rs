use oneline_eyre::eyre::eyre;

use hermes_cli_framework::command::Runnable;
use hermes_cli_framework::output::Output;
use hermes_cosmos_client_components::traits::chain_handle::HasBlockingChainHandle;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
use hermes_relayer_components::chain::traits::queries::client_state::CanQueryClientState;

use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer::chain::requests::{
    IncludeProof, PageRequest, QueryChannelsRequest, QueryClientStateRequest,
    QueryConnectionRequest, QueryHeight,
};
use ibc_relayer_types::core::ics04_channel::channel::State;
use ibc_relayer_types::core::ics24_host::identifier::{ChainId, PortChannelId};

use tracing::{info, warn};

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

        let channels = chain
            .with_blocking_chain_handle(move |chain_handle| {
                let mut channels = chain_handle
                    .query_channels(QueryChannelsRequest {
                        pagination: Some(PageRequest::all()),
                    })
                    .expect("failed QueryChannelsRequest");

                let chain_height = chain_handle
                    .query_latest_height()
                    .expect("failed to query for chain height");

                channels.retain(|channel| {
                    let port_id = &channel.port_id;
                    let channel_id = &channel.channel_id;
                    let chain_id = chain_id.clone();
                    let channel_end = &channel.channel_end;

                    if channel_end.state_matches(&State::Uninitialized) {
                        warn!(
                            "{}/{} on chain {} @ {:?} is uninitialized",
                            port_id, channel_id, chain_id, chain_height,
                        );

                        return false;
                    }

                    let Some(connection_id) = channel.channel_end.connection_hops.first() else {
                        warn!(
                            "missing connection hops for {}/{} on chain {} @ {:?}",
                            port_id, channel_id, chain_id, chain_height,
                        );

                        return false;
                    };

                    if self.show_counterparty || dst_chain_id.is_some() {
                        let connection_id = connection_id.clone();

                        let Ok((connection_end, _)) = chain_handle.query_connection(
                            QueryConnectionRequest {
                                connection_id,
                                height: QueryHeight::Specific(chain_height),
                            },
                            IncludeProof::No,
                        ) else {
                            warn!(
                                "missing connection end for {}/{} on chain {} @ {:?}",
                                port_id, channel_id, chain_id, chain_height,
                            );

                            return false;
                        };

                        let client_id = connection_end.client_id().clone();
                        let Ok((client_state, _)) = chain_handle.query_client_state(
                            QueryClientStateRequest {
                                client_id,
                                height: QueryHeight::Specific(chain_height),
                            },
                            IncludeProof::No,
                        ) else {
                            warn!(
                                "missing client state for {}/{} on chain {} @ {:?}",
                                port_id, channel_id, chain_id, chain_height,
                            );

                            return false;
                        };

                        let cid = client_state.chain_id().clone();

                        if let Some(dst_chain_id) = &dst_chain_id {
                            cid == *dst_chain_id
                        } else {
                            true
                        }
                    } else {
                        true
                    }
                });

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

                Ok(channels)
            })
            .await
            .map_err(|e| eyre!("Failed to query channels for host chain: {e}"))?;

        Ok(Output::success(channels))
    }
}
