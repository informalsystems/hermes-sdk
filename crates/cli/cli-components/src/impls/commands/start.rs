use std::marker::PhantomData;

use cgp::extra::run::CanRun;
use cgp::prelude::*;
use hermes_error::traits::wrap::CanWrapError;
use hermes_logging_components::traits::has_logger::HasLogger;
use hermes_logging_components::traits::logger::CanLog;
use hermes_logging_components::types::level::LevelInfo;
use hermes_relayer_components::build::traits::builders::birelay_builder::CanBuildBiRelay;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::chain::traits::types::ibc::HasClientIdType;
use hermes_relayer_components::multi::traits::chain_at::HasChainTypeAt;

use crate::traits::build::CanLoadBuilder;
use crate::traits::command::{CommandRunner, CommandRunnerComponent};
use crate::traits::output::CanProduceOutput;
use crate::traits::parse::CanParseArg;

#[derive(Debug, clap::Parser, HasField)]
pub struct StartRelayerArgs {
    /// Identifier of chain A
    #[clap(
        long = "chain-id-a",
        required = true,
        value_name = "CHAIN_ID_A",
        help_heading = "REQUIRED"
    )]
    chain_id_a: String,

    /// Identifier of client A
    #[clap(
        long = "client-id-a",
        required = true,
        value_name = "CLIENT_ID_A",
        help_heading = "REQUIRED"
    )]
    client_id_a: String,

    /// Identifier of chain B
    #[clap(
        long = "chain-id-b",
        required = true,
        value_name = "CHAIN_ID_B",
        help_heading = "REQUIRED"
    )]
    chain_id_b: String,

    /// Identifier of client B
    #[clap(
        long = "client-id-b",
        required = true,
        value_name = "CLIENT_ID_B",
        help_heading = "REQUIRED"
    )]
    client_id_b: String,
}

#[new_cgp_provider(CommandRunnerComponent)]
impl<App, Args, Build, BiRelay, ChainA, ChainB, TagA, TagB> CommandRunner<App, Args>
    for RunStartRelayerCommand<TagA, TagB>
where
    App: CanLoadBuilder<Builder = Build>
        + HasLogger
        + CanProduceOutput<&'static str>
        + CanParseArg<Args, symbol!("chain_id_a"), Parsed = ChainA::ChainId>
        + CanParseArg<Args, symbol!("client_id_a"), Parsed = ChainA::ClientId>
        + CanParseArg<Args, symbol!("chain_id_b"), Parsed = ChainB::ChainId>
        + CanParseArg<Args, symbol!("client_id_b"), Parsed = ChainB::ClientId>
        + CanRaiseAsyncError<Build::Error>
        + CanRaiseAsyncError<BiRelay::Error>
        + CanWrapError<&'static str>,
    Args: Async,
    App::Logger: CanLog<LevelInfo>,
    Build: CanBuildBiRelay<TagA, TagB, BiRelay = BiRelay>
        + HasChainTypeAt<TagA, Chain = ChainA>
        + HasChainTypeAt<TagB, Chain = ChainB>,
    BiRelay: CanRun,
    ChainA: HasChainIdType + HasClientIdType<ChainB>,
    ChainB: HasChainIdType + HasClientIdType<ChainA>,
{
    async fn run_command(app: &App, args: &Args) -> Result<App::Output, App::Error> {
        let logger = app.logger();
        let builder = app.load_builder().await?;

        let chain_id_a = app.parse_arg(args, PhantomData::<symbol!("chain_id_a")>)?;
        let client_id_a = app.parse_arg(args, PhantomData::<symbol!("client_id_a")>)?;

        let chain_id_b = app.parse_arg(args, PhantomData::<symbol!("chain_id_b")>)?;
        let client_id_b = app.parse_arg(args, PhantomData::<symbol!("client_id_b")>)?;

        let birelay = builder
            .build_birelay(&chain_id_a, &chain_id_b, &client_id_a, &client_id_b)
            .await
            .map_err(App::raise_error)?;

        logger
            .log(
                &format!("Relaying between {} and {}...", chain_id_a, chain_id_b,),
                &LevelInfo,
            )
            .await;

        birelay
            .run()
            .await
            .map_err(|e| App::wrap_error("Relayer failed to start", App::raise_error(e)))?;

        Ok(app.produce_output("Relayer exited successfully."))
    }
}
