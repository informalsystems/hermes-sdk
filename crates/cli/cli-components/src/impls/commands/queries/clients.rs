use core::fmt::Debug;
use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_logging_components::traits::has_logger::HasLogger;
use hermes_logging_components::traits::logger::CanLog;
use hermes_logging_components::types::level::LevelInfo;
use hermes_relayer_components::build::traits::builders::chain_builder::CanBuildChain;
use hermes_relayer_components::chain::traits::queries::client_state::CanQueryAllClientStates;
use hermes_relayer_components::chain::traits::queries::client_state::CanQueryAllClientStatesWithLatestHeight;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::ibc::HasClientIdType;
use hermes_relayer_components::multi::traits::chain_at::HasChainTypeAt;
use hermes_relayer_components::multi::types::index::Index;

use crate::traits::build::CanLoadBuilder;
use crate::traits::command::CommandRunner;
use crate::traits::output::{CanProduceOutput, HasOutputType};
use crate::traits::parse::CanParseArg;

pub struct RunQueryClientsCommand;

#[derive(Debug, clap::Parser, HasField)]
pub struct QueryClients {
    #[clap(
        long = "host-chain",
        required = true,
        value_name = "HOST_CHAIN_ID",
        help_heading = "REQUIRED",
        help = "Identifier of the host chain to query"
    )]
    host_chain_id: String,

    #[clap(
        long = "reference-chain",
        value_name = "REFERENCE_CHAIN_ID",
        help = "Only show clients that reference this chain"
    )]
    reference_chain_id: Option<String>,
}

impl<App, Args, Build, Chain, Counterparty> CommandRunner<App, Args> for RunQueryClientsCommand
where
    App: CanLoadBuilder<Builder = Build>
        + HasOutputType
        + HasLogger
        + CanProduceOutput<Vec<Chain::ClientId>>
        + CanParseArg<Args, symbol!("host_chain_id"), Parsed = Chain::ChainId>
        + CanParseArg<Args, symbol!("reference_chain_id"), Parsed = Chain::ChainId>
        + CanRaiseError<Build::Error>
        + CanRaiseError<Chain::Error>,
    Build: CanBuildChain<Index<0>, Chain = Chain> + HasChainTypeAt<Index<1>, Chain = Counterparty>,
    Chain: HasChainIdType + HasErrorType + HasComponents + HasClientIdType<Counterparty>,
    Counterparty:
        HasClientStateType<Counterparty> + CanQueryAllClientStates<Counterparty> + HasComponents,
    Args: Async,
    App::Logger: CanLog<LevelInfo>,
{
    async fn run_command(app: &App, args: &Args) -> Result<App::Output, App::Error> {
        let builder = app.load_builder().await?;
        let logger = app.logger();

        let host_chain_id = app.parse_arg(args, PhantomData::<symbol!("host_chain_id")>)?;
        let reference_chain_id =
            app.parse_arg(args, PhantomData::<symbol!("reference_chain_id")>)?;

        let chain = builder
            .build_chain(PhantomData::<Index<0>>, &host_chain_id)
            .await
            .map_err(App::raise_error)?;

        let clients = query_all_client_states::<Chain, Counterparty>(
            &chain,
            &host_chain_id,
            &reference_chain_id,
        )
        .await?;

        for client in clients.iter() {
            logger
                .log(
                    &format!(
                        "- {}: {} -> {}",
                        client.client_id,
                        &host_chain_id,
                        client.client_state.chain_id()
                    ),
                    &LevelInfo,
                )
                .await;
        }

        Ok(app.produce_output(clients))
    }
}

async fn query_all_client_states<Chain, Counterparty>(
    chain: &Chain,
    host_chain_id: &Chain::ChainId,
    reference_chain_id: Option<&Counterparty::ChainId>,
) -> Result<Vec<Chain::ClientId>, Chain::Error>
where
    Chain: CanQueryAllClientStatesWithLatestHeight<Counterparty> + HasChainIdType + HasErrorType,
    Counterparty: HasClientIdType<Counterparty> + HasChainIdType + HasClientStateType<Chain>,
{
    let mut clients = chain
        .query_all_client_states_with_latest_height()
        .await
        .map_error(Chain::Error)?
        .into_iter()
        .map(|(client_id, client_state)| Client::<Chain, Counterparty> {
            client_id,
            client_state,
        })
        .collect::<Vec<_>>();

    // info!("Found {} clients on chain `{host_chain_id}`", clients.len());

    if let Some(reference_chain_id) = reference_chain_id {
        clients.retain(|client| client.client_state.chain_id() == reference_chain_id);

        // info!(
        //     "Found {} clients that reference `{reference_chain_id}`",
        //     clients.len()
        // );
    }

    Ok(clients)
}
