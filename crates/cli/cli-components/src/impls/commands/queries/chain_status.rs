use core::fmt::Display;
use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_relayer_components::build::traits::builders::chain_builder::CanBuildChain;
use hermes_relayer_components::chain::traits::queries::chain_status::CanQueryChainStatus;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::multi::types::index::Index;

use crate::traits::build::CanLoadBuilder;
use crate::traits::command::CommandRunner;
use crate::traits::output::CanProduceOutput;
use crate::traits::parse::CanParseArg;

pub struct RunQueryChainStatusCommand;

#[derive(Debug, clap::Parser, HasField)]
pub struct QueryChainStatusArgs {
    #[clap(
        long = "chain",
        required = true,
        value_name = "CHAIN_ID",
        help_heading = "REQUIRED",
        help = "Identifier of the chain to query"
    )]
    chain_id: String,
}

impl<App, Args, Build, Chain> CommandRunner<App, Args> for RunQueryChainStatusCommand
where
    App: CanLoadBuilder<Builder = Build>
        + CanProduceOutput<Chain::ChainStatus>
        + CanParseArg<Args, symbol!("chain_id"), Parsed = Chain::ChainId>
        + CanRaiseError<Build::Error>
        + CanRaiseError<Chain::Error>,
    Build: CanBuildChain<Index<0>, Chain = Chain>,
    Chain: HasChainIdType + CanQueryChainStatus,
    Args: Async,
    Chain::ChainStatus: Display,
{
    async fn run_command(app: &App, args: &Args) -> Result<App::Output, App::Error> {
        let chain_id = app.parse_arg(args, PhantomData::<symbol!("chain_id")>)?;

        let builder = app.load_builder().await?;

        let chain = builder
            .build_chain(PhantomData, &chain_id)
            .await
            .map_err(App::raise_error)?;

        let chain_status = chain.query_chain_status().await.map_err(App::raise_error)?;

        Ok(app.produce_output(chain_status))
    }
}