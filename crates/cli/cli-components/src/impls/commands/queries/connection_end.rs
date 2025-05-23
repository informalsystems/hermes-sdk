use core::marker::PhantomData;

use cgp::core::field::Index;
use hermes_core::relayer_components::build::traits::builders::chain_builder::CanBuildChain;
use hermes_core::relayer_components::chain::traits::{
    CanQueryChainHeight, CanQueryConnectionEnd, HasChainIdType,
};
use hermes_core::relayer_components::multi::traits::chain_at::HasChainTypeAt;
use hermes_prelude::*;

use crate::traits::{
    CanLoadBuilder, CanParseArg, CanProduceOutput, CommandRunner, CommandRunnerComponent,
    HasOutputType,
};

pub struct RunQueryConnectionEndCommand;

#[derive(Debug, clap::Parser, HasField)]
pub struct QueryConnectionEndArgs {
    #[clap(
        long = "chain",
        required = true,
        value_name = "CHAIN_ID",
        help_heading = "REQUIRED",
        help = "Identifier of the chain to query"
    )]
    chain_id: String,

    #[clap(
        long = "connection",
        visible_alias = "conn",
        required = true,
        value_name = "CONNECTION_ID",
        help_heading = "REQUIRED",
        help = "Identifier of the connection to query"
    )]
    connection_id: String,

    #[clap(
        long = "height",
        value_name = "HEIGHT",
        help = "Height of the state to query. Leave unspecified for latest height."
    )]
    height: Option<String>,
}

#[cgp_provider(CommandRunnerComponent)]
impl<App, Args, Build, Chain, Counterparty> CommandRunner<App, Args>
    for RunQueryConnectionEndCommand
where
    App: CanLoadBuilder<Builder = Build>
        + HasOutputType
        + CanProduceOutput<Chain::ConnectionEnd>
        + CanParseArg<Args, symbol!("chain_id"), Parsed = Chain::ChainId>
        + CanParseArg<Args, symbol!("connection_id"), Parsed = Chain::ConnectionId>
        + CanParseArg<Args, symbol!("height"), Parsed = Option<Chain::Height>>
        + CanRaiseAsyncError<Build::Error>
        + CanRaiseAsyncError<Chain::Error>,
    Build: CanBuildChain<Index<0>, Chain = Chain> + HasChainTypeAt<Index<1>, Chain = Counterparty>,
    Chain: HasChainIdType + CanQueryChainHeight + CanQueryConnectionEnd<Counterparty>,
    Args: Async,
{
    async fn run_command(app: &App, args: &Args) -> Result<App::Output, App::Error> {
        let chain_id = app.parse_arg(args, PhantomData::<symbol!("chain_id")>)?;
        let connection_id = app.parse_arg(args, PhantomData::<symbol!("connection_id")>)?;
        let m_query_height = app.parse_arg(args, PhantomData::<symbol!("height")>)?;

        let builder = app.load_builder().await?;

        let chain = builder
            .build_chain(PhantomData::<Index<0>>, &chain_id)
            .await
            .map_err(App::raise_error)?;

        let query_height = match m_query_height {
            Some(query_height) => query_height,
            None => chain.query_chain_height().await.map_err(App::raise_error)?,
        };

        let connection_end = chain
            .query_connection_end(&connection_id, &query_height)
            .await
            .map_err(App::raise_error)?;

        Ok(app.produce_output(connection_end))
    }
}
