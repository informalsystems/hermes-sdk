use oneline_eyre::eyre::Context;
use tracing::info;

use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::{json, Output};
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
use hermes_relayer_components::chain::traits::queries::client_state::CanQueryClientStates;
use ibc_relayer_types::core::ics02_client::client_state::ClientState;
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
            .query_client_states()
            .await
            .wrap_err("Failed to query clients")?;

        info!("Found {} clients on chain `{chain_id}`", clients.len());

        if let Some(reference_chain_id) = &self.reference_chain_id {
            clients.retain(|(_, state)| &state.chain_id() == reference_chain_id);

            info!(
                "Found {} clients that reference `{reference_chain_id}`",
                clients.len()
            );
        }

        if !json() {
            clients.iter().for_each(|(id, state)| {
                if self.verbose {
                    info!("- {id}: {state:#?}",);
                } else {
                    info!("- {id}");
                }
            });
        }

        if json() {
            if self.verbose {
                let clients = clients
                    .into_iter()
                    .map(|(id, state)| {
                        serde_json::json!({
                            "client_id": id,
                            "client_state": state,
                        })
                    })
                    .collect::<Vec<_>>();

                Ok(Output::success(clients))
            } else {
                let client_ids = clients.into_iter().map(|(id, _)| id).collect::<Vec<_>>();
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
