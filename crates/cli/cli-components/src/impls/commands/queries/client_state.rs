use core::fmt::Display;
use core::marker::PhantomData;

use cgp::core::field::Index;
use hermes_core::logging_components::traits::CanLog;
use hermes_core::logging_components::types::LevelInfo;
use hermes_core::relayer_components::build::traits::builders::chain_builder::CanBuildChain;
use hermes_core::relayer_components::chain::traits::{
    CanQueryChainHeight, CanQueryClientState, HasChainIdType, HasClientStateType,
};
use hermes_prelude::*;

use crate::traits::{
    CanLoadBuilder, CanParseArg, CanProduceOutput, CommandRunner, CommandRunnerComponent,
    HasAnyCounterpartyType, HasBuilderType,
};

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

#[cgp_new_provider(CommandRunnerComponent)]
impl<App, Args, Build, Chain, Counterparty> CommandRunner<App, Args> for RunQueryClientStateCommand
where
    App: HasBuilderType<Builder = Build>
        + CanLoadBuilder
        + CanLog<LevelInfo>
        + HasAnyCounterpartyType<AnyCounterparty = Counterparty>
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
    Chain::ClientId: Display,
{
    async fn run_command(app: &App, args: &Args) -> Result<App::Output, App::Error> {
        let chain_id = app.parse_arg(args, PhantomData::<symbol!("chain_id")>)?;
        let client_id = app.parse_arg(args, PhantomData::<symbol!("client_id")>)?;
        let m_height = app.parse_arg(args, PhantomData::<symbol!("height")>)?;

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

        app.log(
            &format!("Found client state for client `{client_id}` on chain `{chain_id}`!"),
            &LevelInfo,
        )
        .await;

        Ok(app.produce_output(client_state))
    }
}
