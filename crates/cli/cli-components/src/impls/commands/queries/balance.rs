use core::fmt::Display;
use core::marker::PhantomData;

use cgp::core::field::Index;
use cgp::prelude::*;
use hermes_relayer_components::build::traits::builders::chain_builder::CanBuildChain;
use hermes_relayer_components::chain::traits::HasChainIdType;
use hermes_test_components::chain::traits::queries::balance::CanQueryBalance;

use crate::traits::build::CanLoadBuilder;
use crate::traits::command::{CommandRunner, CommandRunnerComponent};
use crate::traits::output::CanProduceOutput;
use crate::traits::parse::CanParseArg;

pub struct RunQueryBalanceCommand;

#[derive(Debug, clap::Parser, HasField)]
pub struct QueryBalanceArgs {
    #[clap(
        long = "chain",
        required = true,
        value_name = "CHAIN_ID",
        help_heading = "REQUIRED",
        help = "Identifier of the chain to query"
    )]
    chain_id: String,

    #[clap(
        long = "address",
        required = true,
        value_name = "ADDRESS",
        help_heading = "REQUIRED",
        help = "Wallet address to query for balance"
    )]
    address: String,

    #[clap(
        long = "denom",
        required = true,
        value_name = "DENOM",
        help_heading = "REQUIRED",
        help = "Token denom queried"
    )]
    denom: String,
}

#[cgp_provider(CommandRunnerComponent)]
impl<App, Args, Build, Chain> CommandRunner<App, Args> for RunQueryBalanceCommand
where
    App: CanLoadBuilder<Builder = Build>
        + CanProduceOutput<Chain::Amount>
        + CanParseArg<Args, symbol!("chain_id"), Parsed = Chain::ChainId>
        + CanParseArg<Args, symbol!("address"), Parsed = Chain::Address>
        + CanParseArg<Args, symbol!("denom"), Parsed = Chain::Denom>
        + CanRaiseAsyncError<Build::Error>
        + CanRaiseAsyncError<Chain::Error>,
    Build: CanBuildChain<Index<0>, Chain = Chain>,
    Chain: HasChainIdType + CanQueryBalance,
    Args: Async,
    Chain::Amount: Display,
{
    async fn run_command(app: &App, args: &Args) -> Result<App::Output, App::Error> {
        let chain_id = app.parse_arg(args, PhantomData::<symbol!("chain_id")>)?;
        let address = app.parse_arg(args, PhantomData::<symbol!("address")>)?;
        let denom = app.parse_arg(args, PhantomData::<symbol!("denom")>)?;

        let builder = app.load_builder().await?;

        let chain = builder
            .build_chain(PhantomData, &chain_id)
            .await
            .map_err(App::raise_error)?;

        let balance = chain
            .query_balance(&address, &denom)
            .await
            .map_err(App::raise_error)?;

        Ok(app.produce_output(balance))
    }
}
