use std::error::Error as StdError;
use std::fmt;

use cgp_core::HasErrorType;
use oneline_eyre::eyre::Context;
use tracing::info;

use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::{json, Output};
use hermes_cosmos_client_components::types::tendermint::TendermintClientState;
use hermes_cosmos_relayer::contexts::builder::CosmosBuilder;
use hermes_cosmos_relayer::types::error::BaseError;
use hermes_relayer_components::chain::traits::queries::client_state::CanQueryClientStates;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer_types::core::ics02_client::client_state::ClientState;
use ibc_relayer_types::core::ics24_host::identifier::{ChainId, ClientId};

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
        let clients = query_client_states(
            &chain,
            &self.host_chain_id,
            self.reference_chain_id.as_ref(),
        )
        .await?;

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
                    .into_iter()
                    .map(|client| client.client_id)
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

#[derive(serde::Serialize)]
struct Client<Chain, Counterparty>
where
    Chain: HasIbcChainTypes<Counterparty>,
    Counterparty: HasClientStateType<Chain>,
{
    client_id: Chain::ClientId,
    client_state: Counterparty::ClientState,
}

impl<Chain, Counterparty> fmt::Debug for Client<Chain, Counterparty>
where
    Chain: HasIbcChainTypes<Counterparty>,
    Counterparty: HasClientStateType<Chain>,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Client")
            .field("client_id", &self.client_id)
            .field("client_state", &self.client_state)
            .finish()
    }
}

async fn query_client_states<Chain, Counterparty>(
    chain: &Chain,
    host_chain_id: &ChainId,
    reference_chain_id: Option<&ChainId>,
) -> Result<Vec<Client<Chain, Counterparty>>>
where
    Chain: HasIbcChainTypes<Counterparty, ClientId = ClientId>
        + CanQueryClientStates<Counterparty>
        + HasErrorType,
    Counterparty: HasClientStateType<Chain, ClientState = TendermintClientState>,
    Chain::Error: From<BaseError> + StdError,
{
    let mut clients = chain
        .query_client_states()
        .await
        .wrap_err("Failed to query clients")?
        .into_iter()
        .map(|(client_id, client_state)| Client::<Chain, Counterparty> {
            client_id,
            client_state,
        })
        .collect::<Vec<_>>();

    info!("Found {} clients on chain `{host_chain_id}`", clients.len());

    if let Some(reference_chain_id) = reference_chain_id {
        clients.retain(|client| &client.client_state.chain_id() == reference_chain_id);

        info!(
            "Found {} clients that reference `{reference_chain_id}`",
            clients.len()
        );
    }

    Ok(clients)
}
