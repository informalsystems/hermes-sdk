use std::marker::PhantomData;

use cgp::extra::run::CanRun;
use hermes_core::logging_components::traits::CanLog;
use hermes_core::logging_components::types::LevelInfo;
use hermes_core::relayer_components::birelay::traits::CanAutoBiRelay;
use hermes_core::relayer_components::build::traits::builders::birelay_builder::CanBuildBiRelay;
use hermes_core::relayer_components::chain::traits::{HasChainIdType, HasClientIdType};
use hermes_core::relayer_components::multi::traits::chain_at::HasChainTypeAt;
use hermes_prelude::*;

use crate::traits::{
    CanLoadBuilder, CanParseArg, CanProduceOutput, CommandRunner, CommandRunnerComponent,
};

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

    #[clap(long = "clear-past-blocks", required = false)]
    clear_past_blocks: Option<humantime::Duration>,

    #[clap(long = "stop-after-blocks", required = false)]
    stop_after_blocks: Option<humantime::Duration>,

    #[clap(long = "refresh-rate-a", required = false)]
    refresh_rate_a: Option<humantime::Duration>,

    #[clap(long = "refresh-rate-b", required = false)]
    refresh_rate_b: Option<humantime::Duration>,
}

#[cgp_auto_getter]
pub trait HasClearPacketFields {
    fn clear_past_blocks(&self) -> &Option<humantime::Duration>;

    fn stop_after_blocks(&self) -> &Option<humantime::Duration>;
}

#[cgp_auto_getter]
pub trait HasRefreshClientFields {
    fn refresh_rate_a(&self) -> &Option<humantime::Duration>;

    fn refresh_rate_b(&self) -> &Option<humantime::Duration>;
}

#[cgp_new_provider(CommandRunnerComponent)]
impl<App, Args, Build, BiRelay, ChainA, ChainB, TagA, TagB> CommandRunner<App, Args>
    for RunStartRelayerCommand<TagA, TagB>
where
    App: CanLoadBuilder<Builder = Build>
        + CanLog<LevelInfo>
        + CanProduceOutput<&'static str>
        + CanParseArg<Args, symbol!("chain_id_a"), Parsed = ChainA::ChainId>
        + CanParseArg<Args, symbol!("client_id_a"), Parsed = ChainA::ClientId>
        + CanParseArg<Args, symbol!("chain_id_b"), Parsed = ChainB::ChainId>
        + CanParseArg<Args, symbol!("client_id_b"), Parsed = ChainB::ClientId>
        + CanRaiseAsyncError<Build::Error>
        + CanRaiseAsyncError<BiRelay::Error>
        + CanWrapError<&'static str>,
    Args: Async + HasClearPacketFields + HasRefreshClientFields,
    Build: CanBuildBiRelay<TagA, TagB, BiRelay = BiRelay>
        + HasChainTypeAt<TagA, Chain = ChainA>
        + HasChainTypeAt<TagB, Chain = ChainB>,
    BiRelay: CanRun + CanAutoBiRelay,
    ChainA: HasChainIdType + HasClientIdType<ChainB>,
    ChainB: HasChainIdType + HasClientIdType<ChainA>,
{
    async fn run_command(app: &App, args: &Args) -> Result<App::Output, App::Error> {
        let builder = app.load_builder().await?;

        let chain_id_a = app.parse_arg(args, PhantomData::<symbol!("chain_id_a")>)?;
        let client_id_a = app.parse_arg(args, PhantomData::<symbol!("client_id_a")>)?;

        let chain_id_b = app.parse_arg(args, PhantomData::<symbol!("chain_id_b")>)?;
        let client_id_b = app.parse_arg(args, PhantomData::<symbol!("client_id_b")>)?;

        let clear_past_blocks = *args.clear_past_blocks();
        let stop_after_blocks = *args.stop_after_blocks();

        let refresh_rate_a = *args.refresh_rate_a();
        let refresh_rate_b = *args.refresh_rate_b();

        let birelay = builder
            .build_birelay(
                &chain_id_a,
                &chain_id_b,
                &client_id_a,
                &client_id_b,
                refresh_rate_a.map(Into::into),
                refresh_rate_b.map(Into::into),
            )
            .await
            .map_err(App::raise_error)?;

        app.log(
            &format!("Relaying between {chain_id_a} and {chain_id_b}..."),
            &LevelInfo,
        )
        .await;

        birelay
            .auto_bi_relay(
                clear_past_blocks.map(Into::into),
                stop_after_blocks.map(Into::into),
                refresh_rate_a.map(Into::into),
                refresh_rate_b.map(Into::into),
            )
            .await
            .map_err(|e| App::wrap_error(App::raise_error(e), "Relayer failed to start"))?;

        Ok(app.produce_output("Relayer exited successfully."))
    }
}
