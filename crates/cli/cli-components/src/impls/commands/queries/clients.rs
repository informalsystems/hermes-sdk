use core::fmt::Display;
use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_logging_components::traits::has_logger::HasLogger;
use hermes_logging_components::traits::logger::CanLog;
use hermes_logging_components::types::level::LevelInfo;
use hermes_relayer_components::build::traits::builders::chain_builder::CanBuildChain;
use hermes_relayer_components::chain::traits::queries::client_state::CanQueryAllClientStatesWithLatestHeight;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainId;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;
use hermes_relayer_components::chain::traits::types::ibc::HasClientIdType;
use hermes_relayer_components::multi::traits::chain_at::HasChainTypeAt;
use hermes_relayer_components::multi::types::index::Index;

use crate::traits::build::CanLoadBuilder;
use crate::traits::command::CanRunCommand;
use crate::traits::command::CommandRunner;
use crate::traits::output::CanProduceOutput;
use crate::traits::parse::CanParseArg;

pub struct RunQueryClientsSubCommand;

#[derive(Debug, clap::Subcommand)]
pub enum QueryClientsSubCommand {
    /// Query all clients
    Clients(QueryClientsArgs),
}

impl<App> CommandRunner<App, QueryClientsSubCommand> for RunQueryClientsSubCommand
where
    App: CanRunCommand<QueryClientsArgs>,
{
    async fn run_command(
        app: &App,
        subcommand: &QueryClientsSubCommand,
    ) -> Result<App::Output, App::Error> {
        match subcommand {
            QueryClientsSubCommand::Clients(args) => app.run_command(args).await,
        }
    }
}

pub struct RunQueryClientsCommand;

#[derive(Debug, clap::Parser, HasField)]
pub struct QueryClientsArgs {
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
        + HasLogger
        + CanProduceOutput<Vec<(Chain::ClientId, Counterparty::ClientState)>>
        + CanParseArg<Args, symbol!("host_chain_id"), Parsed = Chain::ChainId>
        + CanParseArg<Args, symbol!("reference_chain_id"), Parsed = Option<Counterparty::ChainId>>
        + CanRaiseError<Build::Error>
        + CanRaiseError<Chain::Error>,
    Build: CanBuildChain<Index<0>, Chain = Chain> + HasChainTypeAt<Index<1>, Chain = Counterparty>,
    Chain: HasChainIdType
        + HasErrorType
        + HasComponents
        + HasClientIdType<Counterparty>
        + CanQueryAllClientStatesWithLatestHeight<Counterparty>,
    Counterparty:
        HasClientIdType<Counterparty> + HasClientStateType<Chain> + HasComponents + HasChainIdType,
    Counterparty::ClientState: HasChainIdType<ChainId = Counterparty::ChainId> + HasChainId,
    Chain::ClientId: Display,
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

        let client_info =
            query_all_client_states::<Chain, Counterparty>(&chain, &reference_chain_id)
                .await
                .map_err(App::raise_error)?;

        for (client_id, client_state) in client_info.iter() {
            logger
                .log(
                    &format!(
                        "- {}: {} -> {}",
                        client_id,
                        &host_chain_id,
                        client_state.chain_id()
                    ),
                    &LevelInfo,
                )
                .await;
        }

        Ok(app.produce_output(client_info))
    }
}

async fn query_all_client_states<Chain, Counterparty>(
    chain: &Chain,
    reference_chain_id: &Option<Counterparty::ChainId>,
) -> Result<Vec<(Chain::ClientId, Counterparty::ClientState)>, Chain::Error>
where
    Chain: CanQueryAllClientStatesWithLatestHeight<Counterparty> + HasErrorType + HasChainIdType,
    Counterparty: HasClientIdType<Counterparty> + HasChainIdType + HasClientStateType<Chain>,
    Counterparty::ClientState: HasChainIdType<ChainId = Counterparty::ChainId> + HasChainId,
{
    let mut client_info = chain
        .query_all_client_states_with_latest_height()
        .await?
        .into_iter()
        .collect::<Vec<_>>();

    if let Some(reference_chain_id) = reference_chain_id {
        client_info.retain(|info| info.1.chain_id() == reference_chain_id);
    }

    Ok(client_info)
}
