use hermes_cli_components::traits::build::CanLoadBuilder;
use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::{json, Output};
use hermes_cosmos_chain_components::traits::chain_handle::HasBlockingChainHandle;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_error::traits::wrap::ErrorWrapper;
use hermes_relayer_components::chain::traits::queries::client_state::CanQueryClientStateWithLatestHeight;
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer::chain::requests::{PageRequest, QueryConnectionsRequest};
use ibc_relayer_types::core::ics02_client::client_state::ClientState;
use ibc_relayer_types::core::ics24_host::identifier::ChainId;
use tracing::{info, warn};

use crate::contexts::app::HermesApp;
use crate::Result;

#[derive(Debug, clap::Parser)]
pub struct QueryConnections {
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
        long = "verbose",
        help = "Enable verbose output, displaying the client for each connection in the response"
    )]
    verbose: bool,
}

impl CommandRunner<HermesApp> for QueryConnections {
    async fn run(&self, app: &HermesApp) -> Result<Output> {
        let builder = app.load_builder().await?;

        let chain = builder.build_chain(&self.chain_id).await?;
        let chain_id = self.chain_id.clone();
        let counterparty_chain_id = self.counterparty_chain_id.clone();
        let verbose = self.verbose;

        let all_connections = chain
            .with_blocking_chain_handle(move |handle| {
                handle
                    .query_connections(QueryConnectionsRequest {
                        pagination: Some(PageRequest::all()),
                    })
                    .map_err(From::from)
            })
            .await
            .wrap_error("Failed to query connections for host chain")?;

        info!(
            "Found {} connections on chain `{chain_id}`",
            all_connections.len()
        );

        let connections = if let Some(filter_chain_id) = counterparty_chain_id {
            let mut connections = Vec::new();

            for connection in all_connections {
                let client_id = connection.end().client_id().to_owned();

                let client_state = <CosmosChain as CanQueryClientStateWithLatestHeight<
                    CosmosChain,
                >>::query_client_state_with_latest_height(
                    &chain, &client_id
                )
                .await;

                let include = match client_state {
                    Ok(client_state) => {
                        let counterparty_chain_id = client_state.chain_id();
                        counterparty_chain_id == filter_chain_id
                    }
                    Err(e) => {
                        warn!("failed to query client state for client `{client_id}`, skipping...");
                        warn!("reason: {e}");

                        false
                    }
                };

                if include {
                    connections.push(connection);
                }
            }

            info!(
                "Found {} connections on chain `{chain_id}` with counterparty chain `{filter_chain_id}`",
                connections.len()
            );

            connections
        } else {
            all_connections
        };

        if json() {
            if verbose {
                Ok(Output::success(connections))
            } else {
                let connection_ids = connections
                    .into_iter()
                    .map(|connection| connection.connection_id)
                    .collect::<Vec<_>>();

                Ok(Output::success(connection_ids))
            }
        } else {
            connections.iter().for_each(|connection| {
                if verbose {
                    info!("- {connection:#?}",);
                } else {
                    info!("- {}", connection.connection_id);
                }
            });

            Ok(Output::success_msg(format!(
                "Total: {} connections",
                connections.len()
            )))
        }
    }
}
