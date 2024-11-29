use std::fmt;

use cgp::core::error::HasErrorType;
use hermes_any_counterparty::contexts::any_counterparty::AnyCounterparty;
use hermes_any_counterparty::types::client_state::AnyClientState;
use hermes_cli_components::traits::build::CanLoadBuilder;
use hermes_cli_framework::command::CommandRunner;
use hermes_cli_framework::output::{json, Output};
use hermes_cosmos_chain_components::types::tendermint::TendermintClientState;
use hermes_cosmos_relayer::contexts::chain::CosmosChain;
use hermes_error::types::Error;
use hermes_relayer_components::chain::traits::queries::client_state::CanQueryAllClientStatesWithLatestHeight;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use ibc_relayer_types::core::ics02_client::client_state::ClientState;
use ibc_relayer_types::core::ics24_host::identifier::{ChainId, ClientId};
use tracing::info;

use crate::contexts::app::HermesApp;
use crate::impls::error_wrapper::ErrorWrapper;
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

impl CommandRunner<HermesApp> for QueryClients {
    async fn run(&self, app: &HermesApp) -> Result<Output> {
        let builder = app.load_builder().await?;

        let chain = builder.build_chain(&self.host_chain_id).await?;
        let clients = query_all_client_states::<CosmosChain, AnyCounterparty>(
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
                    info!(
                        "- {}: {} -> {}",
                        client.client_id,
                        self.host_chain_id,
                        client.client_state.chain_id()
                    );
                }
            });
        }

        if json() {
            if self.verbose {
                Ok(Output::success(clients))
            } else {
                let client_ids = clients
                    .into_iter()
                    .map(|client| {
                        serde_json::json!({
                            "client_id": client.client_id,
                            "host_chain_id": self.host_chain_id,
                            "reference_chain_id": client.client_state.chain_id(),
                        })
                    })
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
    Counterparty::ClientState: fmt::Debug,
    for<'a> Pretty<'a, Counterparty::ClientState>: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Client")
            .field("client_id", &self.client_id)
            .field("client_state", &Pretty(&self.client_state))
            .finish()
    }
}

async fn query_all_client_states<Chain, Counterparty>(
    chain: &Chain,
    host_chain_id: &ChainId,
    reference_chain_id: Option<&ChainId>,
) -> Result<Vec<Client<Chain, Counterparty>>>
where
    Chain: HasIbcChainTypes<Counterparty, ClientId = ClientId>
        + CanQueryAllClientStatesWithLatestHeight<Counterparty>
        + HasErrorType<Error = Error>,
    Counterparty: HasClientStateType<Chain, ClientState = AnyClientState>,
{
    let mut clients = chain
        .query_all_client_states_with_latest_height()
        .await
        .wrap_error("Failed to query clients")?
        .into_iter()
        .map(|(client_id, client_state)| Client::<Chain, Counterparty> {
            client_id,
            client_state,
        })
        .collect::<Vec<_>>();

    info!("Found {} clients on chain `{host_chain_id}`", clients.len());

    if let Some(reference_chain_id) = reference_chain_id {
        clients.retain(|client| client.client_state.chain_id() == reference_chain_id);

        info!(
            "Found {} clients that reference `{reference_chain_id}`",
            clients.len()
        );
    }

    Ok(clients)
}

pub struct Pretty<'a, A: ?Sized>(&'a A);

impl fmt::Debug for Pretty<'_, TendermintClientState> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let client_state = self.0;

        f.debug_struct("ClientState")
            .field("chain_id", &client_state.chain_id.to_string())
            .field("client_type", &client_state.client_type())
            .field("latest_height", &client_state.latest_height.to_string())
            .field("trust_threshold", &client_state.trust_threshold.to_string())
            .field("trusting_period", &client_state.trusting_period)
            .field("unbonding_period", &client_state.unbonding_period)
            .field("max_clock_drift", &client_state.max_clock_drift)
            .field("frozen_height", &client_state.frozen_height)
            .finish()
    }
}

impl fmt::Debug for Pretty<'_, AnyClientState> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            AnyClientState::Tendermint(ref client_state) => Pretty(client_state).fmt(f),
            // AnyClientState::Sovereign(ref client_state) => Pretty(client_state).fmt(f),
        }
    }
}
