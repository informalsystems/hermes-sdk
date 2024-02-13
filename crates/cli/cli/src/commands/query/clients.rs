use hermes_cosmos_relayer::types::error::BaseError;
use oneline_eyre::eyre::Context;
use tracing::info;

use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::{json, Output};
use hermes_cosmos_client_components::traits::chain_handle::HasBlockingChainHandle;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
use ibc_relayer::chain::handle::ChainHandle;
use ibc_relayer::chain::requests::{PageRequest, QueryClientStatesRequest};
use ibc_relayer_types::core::ics24_host::identifier::ChainId;

use crate::Result;

#[derive(Debug, clap::Parser)]
pub struct QueryClients {
    #[clap(
        long = "host-chain",
        required = true,
        value_name = "HOST_CHAIN_ID",
        help_heading = "REQUIRED",
        help = "Identifier of the host chain to query"
    )]
    host_chain_id: ChainId,

    #[clap(
        long = "reference-chain",
        value_name = "REFERENCE_CHAIN_ID",
        help = "Only show clients that reference this chain"
    )]
    reference_chain_id: Option<ChainId>,

    #[clap(
        long = "verbose",
        help = "Enable verbose output, displaying each's client state"
    )]
    verbose: bool,
}

impl CommandRunner<CosmosBuilder> for QueryClients {
    async fn run(&self, builder: &CosmosBuilder) -> Result<Output> {
        let chain = builder.build_chain(&self.host_chain_id).await?;
        let chain_id = self.host_chain_id.clone();

        let mut clients = chain
            .with_blocking_chain_handle(move |handle| {
                handle
                    .query_clients(QueryClientStatesRequest {
                        pagination: Some(PageRequest::all()),
                    })
                    .map_err(|e| BaseError::relayer(e).into())
            })
            .await
            .wrap_err("Failed to query clients")?;

        info!("Found {} clients on chain `{chain_id}`", clients.len());

        if let Some(reference_chain_id) = &self.reference_chain_id {
            clients.retain(|client| &client.client_state.chain_id() == reference_chain_id);

            info!(
                "Found {} clients that reference `{reference_chain_id}`",
                clients.len()
            );
        }

        if !json() {
            clients.iter().for_each(|client| {
                if self.verbose {
                    info!("- {client:#?}",);
                } else {
                    info!("- {}", client.client_id);
                }
            });
        }

        if json() {
            if self.verbose {
                Ok(Output::success(clients))
            } else {
                let client_ids = clients
                    .iter()
                    .map(|c| c.client_id.clone())
                    .collect::<Vec<_>>();

                Ok(Output::success(client_ids))
            }
        } else {
            Ok(Output::success_msg(format!(
                "Total: {} clients",
                clients.len()
            )))
        }
    }
}
