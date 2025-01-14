use core::fmt::Display;
use core::marker::PhantomData;

use cgp::core::field::Index;
use cgp::prelude::*;

use hermes_logging_components::traits::has_logger::HasLogger;
use hermes_logging_components::traits::logger::CanLog;
use hermes_logging_components::types::level::LevelInfo;
use hermes_relayer_components::build::traits::builders::relay_builder::CanBuildRelay;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::chain::traits::types::connection::HasInitConnectionOptionsType;
use hermes_relayer_components::chain::traits::types::ibc::HasClientIdType;
use hermes_relayer_components::multi::traits::chain_at::HasChainTypeAt;
use hermes_relayer_components::relay::traits::chains::HasRelayChains;

use crate::traits::build::CanLoadBuilder;
use crate::traits::command::CommandRunner;
use crate::traits::output::CanProduceOutput;
use crate::traits::parse::CanParseArg;

#[derive(Debug, clap::Parser, HasField)]
pub struct ConnectionCreateArgs {
    #[clap(
        long = "target-chain-id",
        required = true,
        value_name = "TARGET_CHAIN_ID",
        help_heading = "REQUIRED"
    )]
    target_chain_id: String,

    #[clap(
        long = "target-client-id",
        required = true,
        value_name = "TARGET_CLIENT_ID",
        help_heading = "REQUIRED"
    )]
    target_client_id: String,

    #[clap(
        long = "counterparty-chain-id",
        required = true,
        value_name = "COUNTERPARTY_CHAIN_ID",
        help_heading = "REQUIRED"
    )]
    counterparty_chain_id: String,

    #[clap(
        long = "counterparty-client-id",
        required = true,
        value_name = "COUNTERPARTY_CLIENT_ID",
        help_heading = "REQUIRED"
    )]
    counterparty_client_id: String,
}

pub struct RunCreateConnectionCommand;

impl<App, Args, Builder, Chain, Counterparty, Relay> CommandRunner<App, Args>
    for RunCreateConnectionCommand
where
    App: CanLoadBuilder<Builder = Builder>
        + HasLogger
        + CanProduceOutput<&'static str>
        + CanRaiseError<Builder::Error>
        + CanRaiseError<Relay::Error>
        + CanParseArg<Args, symbol!("target_chain_id"), Parsed = Chain::ChainId>
        + CanParseArg<Args, symbol!("counterparty_chain_id"), Parsed = Counterparty::ChainId>
        + CanParseArg<Args, symbol!("target_client_id"), Parsed = Chain::ClientId>
        + CanParseArg<Args, symbol!("counterparty_client_id"), Parsed = Counterparty::ClientId>,
    App::Logger: CanLog<LevelInfo>,
    Builder: CanBuildRelay<Index<0>, Index<1>, Relay = Relay>
        + HasChainTypeAt<Index<0>, Chain = Chain>
        + HasChainTypeAt<Index<1>, Chain = Counterparty>,
    Chain:
        HasChainIdType + HasClientIdType<Counterparty> + HasInitConnectionOptionsType<Counterparty>,
    Chain::InitConnectionOptions: Default,
    Chain::ChainId: Display,
    Chain::ClientId: Display,
    Counterparty::ChainId: Display,
    Counterparty::ClientId: Display,
    Counterparty: HasChainIdType + HasClientIdType<Chain>,
    Relay: HasRelayChains<SrcChain = Chain, DstChain = Counterparty>,
    Args: Async,
{
    async fn run_command(app: &App, args: &Args) -> Result<App::Output, App::Error> {
        let logger = app.logger();
        let builder = app.load_builder().await?;

        let target_chain_id = app.parse_arg(args, PhantomData::<symbol!("target_chain_id")>)?;
        let target_client_id = app.parse_arg(args, PhantomData::<symbol!("target_client_id")>)?;
        let counterparty_chain_id =
            app.parse_arg(args, PhantomData::<symbol!("counterparty_chain_id")>)?;
        let counterparty_client_id =
            app.parse_arg(args, PhantomData::<symbol!("counterparty_client_id")>)?;

        logger
            .log(
                &format!(
                    "Creating connection between {}:{} and {}:{}...",
                    target_chain_id,
                    target_client_id,
                    counterparty_chain_id,
                    counterparty_client_id
                ),
                &LevelInfo,
            )
            .await;

        let relay = builder
            .build_relay(
                PhantomData::<(Index<0>, Index<1>)>,
                &target_chain_id,
                &counterparty_chain_id,
                &target_client_id,
                &counterparty_client_id,
            )
            .await
            .map_err(App::raise_error)?;

        let (target_connection_id, counterparty_connection_id) = relay
            .bootstrap_connection(&Chain::InitConnectionOptions)
            .await
            .map_err(App::raise_error)?;

        logger
            .log(
                &format!(
                    "Connection successfully created between {}:{} and {}:{}",
                    target_chain_id,
                    target_client_id,
                    counterparty_chain_id,
                    counterparty_client_id,
                ),
                &LevelInfo,
            )
            .await;

        Ok(app.produce_output("Done"))
    }
}
