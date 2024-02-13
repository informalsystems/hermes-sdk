use oneline_eyre::eyre::eyre;
use tracing::{info, warn};

use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;
use hermes_cosmos_client_components::traits::chain_handle::HasBlockingChainHandle;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer::chain::requests::{
    IncludeProof, QueryClientStateRequest, QueryConnectionsRequest, QueryHeight,
};
use ibc_relayer_types::core::ics24_host::identifier::ChainId;

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

impl CommandRunner<CosmosBuilder> for QueryConnections {
    async fn run(&self, builder: &CosmosBuilder) -> Result<Output> {
        let chain = builder.build_chain(&self.chain_id).await?;
        let chain_id = self.chain_id.clone();
        let counterparty_chain_id = self.counterparty_chain_id.clone();
        let verbose = self.verbose;

        let connections = chain
            .with_blocking_chain_handle(move |chain_handle| {
                let mut connections =
                    chain_handle.query_connections(QueryConnectionsRequest { pagination: None }).unwrap();

                if let Some(filter_chain_id) = counterparty_chain_id {
                    connections.retain(|connection| {
                        let client_id = connection.end().client_id().to_owned();
                        let chain_height = chain_handle.query_latest_height();

                        let client_state = chain_handle.query_client_state(
                            QueryClientStateRequest {
                                client_id: client_id.clone(),
                                height: QueryHeight::Specific(chain_height.unwrap()),
                            },
                            IncludeProof::No,
                        );

                        match client_state {
                            Ok((client_state, _)) => {
                                let counterparty_chain_id = client_state.chain_id();
                                counterparty_chain_id == filter_chain_id
                            }
                            Err(e) => {
                                warn!("failed to query client state for client {client_id}, skipping...");
                                warn!("reason: {e}");

                                false
                            }
                        }
                    });
                };

                info!("Successfully queried connections on chain `{chain_id}`");

                connections.iter().for_each(|connection| {
                    if verbose {
                        info!("{connection:#?}",);
                    } else {
                        info!("{}", connection.connection_id);
                    }
                });

                Ok(connections)
            })
            .await
            .map_err(|e| eyre!("Failed to query connections for host chain: {e}"))?;

        Ok(Output::success(connections))
    }
}
