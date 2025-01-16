use core::fmt::Display;
use core::marker::PhantomData;

use cgp::prelude::*;
use hermes_logging_components::traits::has_logger::HasLogger;
use hermes_logging_components::traits::logger::CanLog;
use hermes_logging_components::types::level::LevelInfo;
use hermes_relayer_components::build::traits::builders::relay_builder::CanBuildRelay;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::chain::traits::types::connection::HasInitConnectionOptionsType;
use hermes_relayer_components::chain::traits::types::ibc::HasClientIdType;
use hermes_relayer_components::multi::traits::chain_at::HasChainTypeAt;
use hermes_relayer_components::multi::traits::relay_at::HasRelayTypeAt;
use hermes_relayer_components::multi::types::index::Index;
use hermes_relayer_components::relay::impls::connection::bootstrap::CanBootstrapConnection;
use hermes_relayer_components::relay::traits::chains::HasRelayChains;

use crate::traits::build::CanLoadBuilder;
use crate::traits::command::CommandRunner;
use crate::traits::output::{CanProduceOutput, HasOutputType};
use crate::traits::parse::CanParseArg;

pub struct RunCreateConnectionCommand;

#[derive(Debug, clap::Parser, HasField)]
pub struct CreateConnectionArgs {
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

impl<App, Args, Builder, Chain, Counterparty, Relay> CommandRunner<App, Args>
    for RunCreateConnectionCommand
where
    App: HasOutputType + HasAsyncErrorType,
    App: CanLoadBuilder<Builder = Builder>
        + HasLogger
        + CanProduceOutput<&'static str>
        + CanRaiseAsyncError<Builder::Error>
        + CanRaiseAsyncError<Relay::Error>
        + CanParseArg<Args, symbol!("target_chain_id"), Parsed = Chain::ChainId>
        + CanParseArg<Args, symbol!("counterparty_chain_id"), Parsed = Counterparty::ChainId>
        + CanParseArg<Args, symbol!("target_client_id"), Parsed = Chain::ClientId>
        + CanParseArg<Args, symbol!("counterparty_client_id"), Parsed = Counterparty::ClientId>,
    App::Logger: CanLog<LevelInfo>,
    Builder: HasChainTypeAt<Index<0>, Chain = Chain>
        + HasChainTypeAt<Index<1>, Chain = Counterparty>
        + CanBuildRelay<Index<0>, Index<1>, Relay = Relay>
        + HasRelayTypeAt<Index<0>, Index<1>>,
    Chain: HasChainIdType
        + HasClientIdType<Counterparty>
        + HasInitConnectionOptionsType<Counterparty>
        + HasAsyncErrorType,
    Counterparty: HasChainIdType + HasClientIdType<Chain> + HasAsyncErrorType,
    Chain::InitConnectionOptions: Default,
    Chain::ChainId: Display,
    Chain::ClientId: Display,
    Counterparty::ChainId: Display,
    Counterparty::ClientId: Display,
    Relay: CanBootstrapConnection + HasRelayChains<SrcChain = Chain, DstChain = Counterparty>,
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
            .bootstrap_connection(&Default::default())
            .await
            .map_err(App::raise_error)?;

        logger
            .log(
                &format!(
                    "Connection {}:{} successfully created between {}:{} and {}:{}",
                    target_connection_id,
                    counterparty_connection_id,
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
