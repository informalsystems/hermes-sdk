use core::marker::PhantomData;

use hermes_cli_components::traits::{CanLoadBuilder, CommandRunnerComponent};
use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::{json, Output};
use hermes_core::relayer_components::chain::traits::CanQueryClientStateWithLatestHeight;
use hermes_cosmos::chain_components::traits::HasGrpcAddress;
use hermes_cosmos::ibc::core::connection::types::proto::v1::query_client::QueryClient;
use hermes_cosmos::ibc::core::connection::types::proto::v1::QueryConnectionsRequest;
use hermes_cosmos::ibc::core::connection::types::IdentifiedConnectionEnd;
use hermes_cosmos::ibc::core::host::types::identifiers::ChainId;
use hermes_cosmos::relayer::contexts::CosmosChain;
use hermes_prelude::*;
use http::Uri;
use tracing::{info, warn};

use crate::contexts::HermesApp;
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

#[cgp_provider(CommandRunnerComponent)]
impl CommandRunner<HermesApp> for QueryConnections {
    async fn run(&self, app: &HermesApp) -> Result<Output> {
        let builder = app.load_builder().await?;

        let chain = builder.build_chain(&self.chain_id).await?;
        let chain_id = self.chain_id.clone();
        let counterparty_chain_id = self.counterparty_chain_id.clone();
        let verbose = self.verbose;

        let mut client =
            QueryClient::connect(Uri::try_from(&chain.grpc_address().to_string())?).await?;

        let request = tonic::Request::new(QueryConnectionsRequest { pagination: None });

        let response = client.connections(request).await?.into_inner();

        let all_connections = response
            .connections
            .into_iter()
            .filter_map(|co| IdentifiedConnectionEnd::try_from(co.clone()).ok())
            .collect::<Vec<IdentifiedConnectionEnd>>();

        info!(
            "Found {} connections on chain `{chain_id}`",
            all_connections.len()
        );

        let connections = if let Some(filter_chain_id) = counterparty_chain_id {
            let mut connections = Vec::new();

            for connection in all_connections {
                let client_id = connection.end().client_id().to_owned();

                let client_state = chain
                    .query_client_state_with_latest_height(PhantomData::<CosmosChain>, &client_id)
                    .await;

                let include = match client_state {
                    Ok(client_state) => {
                        let counterparty_chain_id = client_state.chain_id();
                        counterparty_chain_id == &filter_chain_id
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
