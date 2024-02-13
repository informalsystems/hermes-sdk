use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::Output;
use hermes_cosmos_client_components::traits::chain_handle::HasBlockingChainHandle;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
use ibc_relayer::chain::handle::ChainHandle;

use hermes_cosmos_relayer::types::error::BaseError;
use ibc_relayer::chain::requests::QueryClientStatesRequest;
use ibc_relayer_types::core::ics24_host::identifier::{ChainId, ClientId};
use oneline_eyre::eyre::eyre;
use serde::Serialize;
use std::fmt::Debug;
use tracing::info;

use crate::Result;

#[derive(Debug, clap::Parser)]
pub struct QueryClients {
    /// Identifier of the host chain
    #[clap(
        long = "host-chain",
        required = true,
        value_name = "CHAIN_ID",
        help_heading = "REQUIRED",
        help = "Identifier of the chain to query"
    )]
    chain_id: ChainId,

    /// Identifier of the reference chain
    #[clap(
        long = "reference-chain",
        value_name = "REFERENCE_CHAIN_ID",
        help = "Filter for clients which target a specific chain id "
    )]
    source_chain_id: Option<ChainId>,

    /// Omit printing the reference (or target) chain for each client
    #[clap(
        long = "omit-chain-ids",
        help = "Omit printing the reference (or target) chain for each client"
    )]
    omit_chain_ids: bool,
}

#[derive(Debug, Serialize)]
struct ClientChain {
    client_id: ClientId,
    chain_id: ChainId,
}

impl QueryClients {
    async fn execute(&self, builder: &CosmosBuilder) -> Result<Vec<ClientChain>> {
        let chain = builder.build_chain(&self.chain_id).await?;
        let counterparty_chain_id = self.source_chain_id.clone();

        let identified_clients = chain
            .with_blocking_chain_handle(move |chain_handle| {
                let clients = chain_handle
                    .query_clients(QueryClientStatesRequest { pagination: None })
                    .map_err(|e| BaseError::generic(eyre!("failed to query clients: {}", e)))?;
                Ok(clients)
            })
            .await?;

        match counterparty_chain_id {
            Some(source_chain_id) => {
                let clients = identified_clients
                    .into_iter()
                    .filter(|cs| cs.client_state.chain_id().eq(&source_chain_id))
                    .map(|cs| ClientChain {
                        client_id: cs.client_id,
                        chain_id: cs.client_state.chain_id().to_owned(),
                    })
                    .collect();
                Ok(clients)
            }
            None => {
                let clients = identified_clients
                    .into_iter()
                    .map(|cs| ClientChain {
                        client_id: cs.client_id,
                        chain_id: cs.client_state.chain_id().to_owned(),
                    })
                    .collect();
                Ok(clients)
            }
        }
    }
}

impl CommandRunner<CosmosBuilder> for QueryClients {
    async fn run(&self, builder: &CosmosBuilder) -> Result<Output> {
        match self.execute(builder).await {
            Ok(clients) => {
                if self.omit_chain_ids || self.source_chain_id.is_some() {
                    let out: Vec<ClientId> = clients.into_iter().map(|cs| cs.client_id).collect();
                    Ok(Output::success(out))
                } else {
                    Ok(Output::success(clients))
                }
            }
            Err(e) => {
                info!("Failed to query clients on chain `{}`", self.chain_id);
                Ok(Output::error(e))
            }
        }
    }
}
