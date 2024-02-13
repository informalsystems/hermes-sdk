use oneline_eyre::eyre::eyre;
use oneline_eyre::eyre::Context;
use serde::{Deserialize, Serialize};

use hermes_cli_framework::command::Runnable;
use hermes_cli_framework::output::Output;

use hermes_cosmos_client_components::traits::chain_handle::HasBlockingChainHandle;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
use hermes_cosmos_relayer::types::error::BaseError;
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainHeight;

use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer::chain::requests::{
    IncludeProof, QueryChannelRequest, QueryClientStateRequest, QueryConnectionRequest, QueryHeight,
};
use ibc_relayer::client_state::AnyClientState;
use ibc_relayer_types::core::ics03_connection::connection::ConnectionEnd;
use ibc_relayer_types::core::ics04_channel::channel::{ChannelEnd, State};
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

    #[clap(
        long = "verbose",
        help = "Enable verbose output, displaying details about all channels, connections, and clients"
    )]
    verbose: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChannelEnds {
    pub channel_end: ChannelEnd,
    pub connection_end: ConnectionEnd,
    pub client_state: AnyClientState,
    pub counterparty_channel_end: ChannelEnd,
    pub counterparty_connection_end: ConnectionEnd,
    pub counterparty_client_state: AnyClientState,
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

impl Runnable for QueryChannelEnds {
    async fn run(&self, builder: CosmosBuilder) -> Result<Output> {
        let chain = builder.build_chain(&self.chain_id).await?;
        let chain_id = self.chain_id.clone();
        let channel_id = self.channel_id.clone();
        let port_id = self.port_id.clone();
        let channel_id = self.channel_id.clone();

        let query_height = if let Some(height) = self.height {
            let specified_height = Height::new(chain_id.version(), height)
            .map_err(|e| BaseError::generic(eyre!("Failed to create Height with revision number `{}` and revision height `{height}`. Error: {e}", chain_id.version())))?;

            QueryHeight::Specific(specified_height)
        } else {
            QueryHeight::Latest
        };

        let (channel_end, _) = chain
            .with_blocking_chain_handle(move |chain_handle| {
                chain_handle
                    .query_channel(
                        QueryChannelRequest {
                            port_id,
                            channel_id,
                            height: query_height,
                        },
                        IncludeProof::No,
                    )
                    .map_err(|e| BaseError::relayer(e).into())
            })
            .await?;

        if channel_end.state_matches(&State::Uninitialized) {
            return Err(BaseError::generic(eyre!(
                "{port_id}/{channel_id} on chain {chain_id} @ {query_height:?} is uninitialized",
            ))
            .into());
        }

        let Some(connection_id) = channel_end.connection_hops.first() else {
            return Err(BaseError::generic(eyre!(
                "missing connection hops for {port_id}/{channel_id} on chain {chain_id} @ {query_height:?}",
            )).into());
        };

        let (connection_end, _) = chain
            .with_blocking_chain_handle(move |chain_handle| {
                chain_handle
                    .query_connection(
                        QueryConnectionRequest {
                            connection_id: connection_id.clone(),
                            height: query_height,
                        },
                        IncludeProof::No,
                    )
                    .map_err(|e| BaseError::relayer(e).into())
            })
            .await?;

        let client_id = connection_end.client_id().clone();

        let (client_state, _) = chain
            .with_blocking_chain_handle(move |chain_handle| {
                chain_handle
                    .query_client_state(
                        QueryClientStateRequest {
                            client_id: client_id.clone(),
                            height: query_height,
                        },
                        IncludeProof::No,
                    )
                    .map_err(|e| BaseError::relayer(e).into())
            })
            .await?;

        let channel_counterparty = channel_end.counterparty().clone();
        let connection_counterparty = connection_end.counterparty().clone();
        let counterparty_client_id = connection_counterparty.client_id().clone();

        let Some(counterparty_connection_id) = connection_counterparty.connection_id else {
            return Err(BaseError::generic(eyre!(
                "connection end for {port_id}/{channel_id} on chain {chain_id} @ {query_height:?} does not have counterparty connection id {connection_end:?}",
            )).into());
        };

        let counterparty_port_id = channel_counterparty.port_id().clone();

        let Some(counterparty_channel_id) = channel_counterparty.channel_id else {
            return Err(BaseError::generic(eyre!(
                "channel end for {port_id}/{channel_id} on chain {chain_id} @ {query_height:?} does not have counterparty channel id {channel_end:?}",
            )).into());
        };

        let counterparty_chain_id = client_state.chain_id();
        // let counterparty_chain = builder.build_chain(&counterparty_chain_id).await?;
        // let counterparty_chain_height_query = QueryHeight::Specific(
        //     counterparty_chain
        //         .query_chain_height()
        //         .await
        //         .wrap_err("Failed to query latest chain height")?,
        // );

        // let (counterparty_connection_end, _) = chain
        //     .with_blocking_chain_handle(move |chain_handle| {
        //         chain_handle
        //             .query_connection(
        //                 QueryConnectionRequest {
        //                     connection_id: counterparty_connection_id.clone(),
        //                     height: counterparty_chain_height_query,
        //                 },
        //                 IncludeProof::No,
        //             )
        //             .map_err(|e| BaseError::relayer(e).into())
        //     })
        //     .await?;

        // let (counterparty_client_state, _) = chain
        //     .with_blocking_chain_handle(move |chain_handle| {
        //         chain_handle
        //             .query_client_state(
        //                 QueryClientStateRequest {
        //                     client_id: counterparty_client_id.clone(),
        //                     height: counterparty_chain_height_query,
        //                 },
        //                 IncludeProof::No,
        //             )
        //             .map_err(|e| BaseError::relayer(e).into())
        //     })
        //     .await?;

        // let (counterparty_channel_end, _) = chain
        //     .with_blocking_chain_handle(move |chain_handle| {
        //         chain_handle
        //             .query_channel(
        //                 QueryChannelRequest {
        //                     port_id: counterparty_port_id.clone(),
        //                     channel_id: counterparty_channel_id.clone(),
        //                     height: counterparty_chain_height_query,
        //                 },
        //                 IncludeProof::No,
        //             )
        //             .map_err(|e| BaseError::relayer(e).into())
        //     })
        //     .await?;

        // if self.verbose {
        //     Ok(Output::success(ChannelEnds {
        //         channel_end,
        //         connection_end,
        //         client_state,
        //         counterparty_channel_end,
        //         counterparty_connection_end,
        //         counterparty_client_state,
        //     }))
        // } else {
        //     Ok(Output::success(ChannelEndsSummary {
        //         chain_id: chain_id.clone(),
        //         client_id,
        //         connection_id: connection_id.clone(),
        //         channel_id: channel_id.clone(),
        //         port_id: port_id.clone(),
        //         counterparty_chain_id,
        //         counterparty_client_id,
        //         counterparty_connection_id,
        //         counterparty_channel_id,
        //         counterparty_port_id,
        //     }))
        // }

        Ok(Output::success(ChannelEndsSummary {
            chain_id: chain_id.clone(),
            client_id: connection_end.client_id().clone(),
            connection_id: connection_id.clone(),
            channel_id: self.channel_id.clone(),
            port_id: self.port_id.clone(),
            counterparty_chain_id,
            counterparty_client_id,
            counterparty_connection_id,
            counterparty_channel_id,
            counterparty_port_id,
        }))
    }
}
