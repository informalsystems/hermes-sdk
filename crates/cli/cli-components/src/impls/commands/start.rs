use std::marker::PhantomData;

use cgp_core::prelude::*;
use cgp_core::run::CanRun;
use hermes_relayer_components::build::traits::builders::birelay_builder::CanBuildBiRelay;
use hermes_relayer_components::chain::traits::types::ibc::HasIbcChainTypes;
use hermes_relayer_components::multi::traits::chain_at::HasChainTypeAt;

use crate::traits::build::CanLoadBuilder;
use crate::traits::command::CommandRunner;
use crate::traits::output::CanProduceOutput;
use crate::traits::parse::CanParseArg;

pub struct StartRelayer;

#[derive(Debug, clap::Parser, HasField)]
pub struct Start {
    /// Identifier of chain A
    #[clap(
        long = "chain-a",
        required = true,
        value_name = "CHAIN_ID_A",
        help_heading = "REQUIRED"
    )]
    chain_id_a: String,

    /// Identifier of client A
    #[clap(
        long = "client-a",
        required = true,
        value_name = "CLIENT_ID_A",
        help_heading = "REQUIRED"
    )]
    client_id_a: String,

    /// Identifier of chain B
    #[clap(
        long = "chain-b",
        required = true,
        value_name = "CHAIN_ID_B",
        help_heading = "REQUIRED"
    )]
    chain_id_b: String,

    /// Identifier of client B
    #[clap(
        long = "client-b",
        required = true,
        value_name = "CLIENT_ID_B",
        help_heading = "REQUIRED"
    )]
    client_id_b: String,
}

impl<App, Args, Build, BiRelay, ChainA, ChainB> CommandRunner<App, Args> for StartRelayer
where
    App: CanLoadBuilder<Builder = Build>
        + CanProduceOutput<&'static str>
        + CanParseArg<Args, symbol!("chain_id_a"), Parsed = ChainA::ChainId>
        + CanParseArg<Args, symbol!("client_id_a"), Parsed = ChainA::ClientId>
        + CanParseArg<Args, symbol!("chain_id_b"), Parsed = ChainB::ChainId>
        + CanParseArg<Args, symbol!("client_id_b"), Parsed = ChainB::ClientId>
        + CanRaiseError<Build::Error>
        + CanRaiseError<BiRelay::Error>,
    Args: Async,
    Build: CanBuildBiRelay<0, 1, BiRelay = BiRelay>
        + HasChainTypeAt<0, Chain = ChainA>
        + HasChainTypeAt<1, Chain = ChainB>,
    BiRelay: CanRun,
    ChainA: HasIbcChainTypes<ChainB>,
    ChainB: HasIbcChainTypes<ChainA>,
{
    async fn run_command(app: &App, args: &Args) -> Result<App::Output, App::Error> {
        let builder = app.load_builder().await?;

        let chain_id_a = app.parse_arg(args, PhantomData::<symbol!("chain_id_a")>)?;
        let client_id_a = app.parse_arg(args, PhantomData::<symbol!("client_id_a")>)?;

        let chain_id_b = app.parse_arg(args, PhantomData::<symbol!("chain_id_b")>)?;
        let client_id_b = app.parse_arg(args, PhantomData::<symbol!("client_id_b")>)?;

        let birelay = builder
            .build_birelay(&chain_id_a, &chain_id_b, &client_id_a, &client_id_b)
            .await
            .map_err(App::raise_error)?;

        birelay.run().await.map_err(App::raise_error)?;

        Ok(app.produce_output("Relayer exited successfully."))
    }
}
