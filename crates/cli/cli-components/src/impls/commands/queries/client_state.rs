use core::fmt::Display;
use core::marker::PhantomData;

use cgp::core::field::Index;
use cgp::prelude::*;
use hermes_logging_components::traits::has_logger::HasLogger;
use hermes_logging_components::traits::logger::CanLog;
use hermes_logging_components::types::level::LevelInfo;
use hermes_relayer_components::build::traits::builders::chain_builder::CanBuildChain;
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainHeight;
use hermes_relayer_components::chain::traits::queries::client_state::CanQueryClientState;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::chain::traits::types::client_state::HasClientStateType;

use crate::traits::any_counterparty::HasAnyCounterparty;
use crate::traits::build::{CanLoadBuilder, HasBuilderType};
use crate::traits::command::CommandRunner;
use crate::traits::output::CanProduceOutput;
use crate::traits::parse::CanParseArg;

pub struct RunQueryClientStateCommand;

#[derive(Debug, clap::Parser, HasField)]
pub struct QueryClientStateArgs {
    /// Identifier of the host chain
    #[clap(
        long = "chain",
        required = true,
        value_name = "CHAIN_ID",
        help_heading = "REQUIRED"
    )]
    chain_id: String,

    /// Identifier of the client on the host chain
    #[clap(
        long = "client",
        required = true,
        value_name = "CLIENT_ID",
        help_heading = "REQUIRED"
    )]
    client_id: String,

    #[clap(
        long = "height",
        value_name = "HEIGHT",
        help = "The height at which to query the client state. If not specified, the latest height is used."
    )]
    height: Option<String>,
}

impl<App, Args, Build, Chain, Counterparty> CommandRunner<App, Args> for RunQueryClientStateCommand
where
    App: HasBuilderType<Builder = Build>
        + CanLoadBuilder
        + HasLogger
        + HasAnyCounterparty<AnyCounterparty = Counterparty>
        + CanProduceOutput<Counterparty::ClientState>
        + CanParseArg<Args, symbol!("chain_id"), Parsed = Chain::ChainId>
        + CanParseArg<Args, symbol!("client_id"), Parsed = Chain::ClientId>
        + CanParseArg<Args, symbol!("height"), Parsed = Option<Chain::Height>>
        + CanRaiseAsyncError<Build::Error>
        + CanRaiseAsyncError<Chain::Error>,
    Args: Async,
    Build: CanBuildChain<Index<0>, Chain = Chain>,
    Chain: HasChainIdType + CanQueryChainHeight + CanQueryClientState<Counterparty>,
    Counterparty: HasClientStateType<Chain>,
    App::Logger: CanLog<LevelInfo>,
    Chain::ClientId: Display,
{
    async fn run_command(app: &App, args: &Args) -> Result<App::Output, App::Error> {
        let chain_id = app.parse_arg(args, PhantomData::<symbol!("chain_id")>)?;
        let client_id = app.parse_arg(args, PhantomData::<symbol!("client_id")>)?;
        let m_height = app.parse_arg(args, PhantomData::<symbol!("height")>)?;

        let logger = app.logger();
        let builder = app.load_builder().await?;

        let chain = builder
            .build_chain(PhantomData, &chain_id)
            .await
            .map_err(App::raise_error)?;

        let query_height = match m_height {
            Some(height) => height,
            None => chain.query_chain_height().await.map_err(App::raise_error)?,
        };

        let client_state = chain
            .query_client_state(PhantomData, &client_id, &query_height)
            .await
            .map_err(App::raise_error)?;

        logger
            .log(
                &format!("Found client state for client `{client_id}` on chain `{chain_id}`!"),
                &LevelInfo,
            )
            .await;

        Ok(app.produce_output(client_state))
    }
}
