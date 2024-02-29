use oneline_eyre::eyre::eyre;
use serde::{Deserialize, Serialize};

use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;

use hermes_cosmos_client_components::traits::chain_handle::HasBlockingChainHandle;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;

use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer::chain::requests::{
    IncludeProof, QueryChannelRequest, QueryClientStateRequest, QueryConnectionRequest, QueryHeight,
};
use ibc_relayer_types::core::ics04_channel::channel::State;
use ibc_relayer_types::core::ics24_host::identifier::{
    ChainId, ChannelId, ClientId, ConnectionId, PortId,
};
use ibc_relayer_types::Height;

use crate::Result;

#[derive(Debug, clap::Parser)]
pub struct QueryChannelEnds {
    #[clap(
        long = "chain",
        required = true,
        value_name = "CHAIN_ID",
        help_heading = "REQUIRED",
        help = "Identifier of the chain to query"
    )]
    chain_id: ChainId,

    #[clap(
        long = "port",
        required = true,
        value_name = "PORT_ID",
        help_heading = "REQUIRED",
        help = "Identifier of the port to query"
    )]
    port_id: PortId,

    #[clap(
        long = "channel",
        required = true,
        value_name = "CHANNEL_ID",
        help_heading = "REQUIRED",
        help = "Identifier of the channel to query"
    )]
    channel_id: ChannelId,

    #[clap(
        long = "height",
        value_name = "HEIGHT",
        help = "Height of the state to query. Leave unspecified for latest height."
    )]
    height: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChannelEndsSummary {
    chain_id: ChainId,
    client_id: ClientId,
    connection_id: ConnectionId,
    channel_id: ChannelId,
    port_id: PortId,
    counterparty_chain_id: ChainId,
    counterparty_client_id: ClientId,
    counterparty_connection_id: ConnectionId,
    counterparty_channel_id: ChannelId,
    counterparty_port_id: PortId,
}

impl CommandRunner<CosmosBuilder> for QueryChannelEnds {
    async fn run(&self, builder: &CosmosBuilder) -> Result<Output> {
        let chain_id = self.chain_id.clone();
        let channel_id = self.channel_id.clone();
        let port_id = self.port_id.clone();
        let height = self.height;

        let chain = builder.build_chain(&chain_id).await?;

        let query_height = if let Some(height) = height {
            let specified_height = Height::new(chain_id.version(), height)
                .map_err(|e| eyre!("Failed to create Height with revision number `{}` and revision height `{height}`: {e}", chain_id.version()))?;

            QueryHeight::Specific(specified_height)
        } else {
            QueryHeight::Latest
        };

        let channel_ends_summary = chain
            .with_blocking_chain_handle(move |chain_handle| {
                let Ok((channel_end , _)) = chain_handle
                    .query_channel(
                        QueryChannelRequest {
                            port_id: port_id.clone(),
                            channel_id: channel_id.clone(),
                            height: query_height,
                        },
                        IncludeProof::No,
                    ) else {
                        return Err(eyre!(
                            "failed to query channel end for {port_id}/{channel_id} on chain {chain_id} @ {query_height:?}"
                        ).into());
                };

                if channel_end.state_matches(&State::Uninitialized) {
                    return Err(eyre!(
                        "{port_id}/{channel_id} on chain {chain_id} @ {query_height:?} is uninitialized",
                    )
                    .into());
                }

                let Some(connection_id) = channel_end.connection_hops.first() else {
                    return Err(eyre!(
                        "missing connection hops for {port_id}/{channel_id} on chain {chain_id} @ {query_height:?}",
                    ).into());
                };

                let Ok((connection_end, _)) = chain_handle
                    .query_connection(
                        QueryConnectionRequest {
                            connection_id: connection_id.clone(),
                            height: query_height,
                        },
                        IncludeProof::No,
                    ) else {
                        return Err(eyre!(
                            "failed to query connection end for {port_id}/{channel_id} on chain {chain_id} @ {query_height:?}"
                        ).into());
                };

                let client_id = connection_end.client_id().clone();

                let Ok((client_state, _)) = chain_handle
                    .query_client_state(
                        QueryClientStateRequest {
                            client_id: client_id.clone(),
                            height: query_height,
                        },
                        IncludeProof::No,
                    ) else {
                        return Err(eyre!(
                            "failed to query client state for {port_id}/{channel_id} on chain {chain_id} @ {query_height:?}"
                        ).into());
                };

                let channel_counterparty = channel_end.counterparty().clone();
                let connection_counterparty = connection_end.counterparty().clone();
                let counterparty_client_id = connection_counterparty.client_id().clone();

                let Some(counterparty_connection_id) = connection_counterparty.connection_id else {
                    return Err(eyre!(
                        "connection end for {port_id}/{channel_id} on chain {chain_id} @ {query_height:?} does not have counterparty connection id {connection_end:?}",
                    ).into());
                };

                let counterparty_port_id = channel_counterparty.port_id().clone();

                let Some(counterparty_channel_id) = channel_counterparty.channel_id else {
                    return Err(eyre!(
                        "channel end for {port_id}/{channel_id} on chain {chain_id} @ {query_height:?} does not have counterparty channel id {channel_end:?}",
                    ).into());
                };

                let counterparty_chain_id = client_state.chain_id();

                Ok(ChannelEndsSummary {
                    chain_id,
                    client_id,
                    connection_id: connection_id.clone(),
                    channel_id,
                    port_id,
                    counterparty_chain_id,
                    counterparty_client_id,
                    counterparty_connection_id,
                    counterparty_channel_id,
                    counterparty_port_id,
                })
            })
            .await;

        match channel_ends_summary {
            Ok(summary) => Ok(Output::success(summary)),
            Err(e) => Err(e),
        }
    }
}
