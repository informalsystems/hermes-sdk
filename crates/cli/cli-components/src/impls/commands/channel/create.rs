use core::fmt::Display;
use core::marker::PhantomData;

use cgp::core::field::Index;
use cgp::prelude::*;
use hermes_logging_components::traits::has_logger::HasLogger;
use hermes_logging_components::traits::logger::CanLog;
use hermes_logging_components::types::level::LevelInfo;
use hermes_relayer_components::build::traits::builders::relay_builder::CanBuildRelay;
use hermes_relayer_components::chain::traits::types::chain_id::HasChainIdType;
use hermes_relayer_components::chain::traits::types::channel::HasInitChannelOptionsType;
use hermes_relayer_components::chain::traits::types::ibc::{HasClientIdType, HasIbcChainTypes};
use hermes_relayer_components::multi::traits::chain_at::HasChainTypeAt;
use hermes_relayer_components::multi::traits::relay_at::HasRelayTypeAt;
use hermes_relayer_components::relay::impls::channel::bootstrap::CanBootstrapChannel;
use hermes_relayer_components::relay::traits::chains::HasRelayChains;

use crate::traits::build::CanLoadBuilder;
use crate::traits::command::CommandRunner;
use crate::traits::output::{CanProduceOutput, HasOutputType};
use crate::traits::parse::CanParseArg;

pub struct RunCreateChannelCommand;

#[derive(Debug, clap::Parser, HasField)]
pub struct CreateChannelArgs {
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
        long = "target-connection-id",
        required = true,
        value_name = "TARGET_CONNECTION_ID",
        help_heading = "REQUIRED"
    )]
    target_connection_id: String,

    #[clap(long = "target-port-id", value_name = "TARGET_PORT_ID")]
    target_port_id: String,

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

    #[clap(long = "counterparty-port-id", value_name = "COUNTERPARTY_PORT_ID")]
    counterparty_port_id: String,

    #[clap(long = "ordering", value_name = "ORDERING")]
    ordering: String,

    #[clap(long = "version", value_name = "VERSION")]
    version: String,
}

impl<App, Args, Builder, Chain, Counterparty, Relay> CommandRunner<App, Args>
    for RunCreateChannelCommand
where
    App: CanLoadBuilder<Builder = Builder>
        + HasOutputType
        + HasErrorType
        + HasLogger
        + CanProduceOutput<&'static str>
        + CanRaiseError<Builder::Error>
        + CanRaiseError<Relay::Error>
        + CanParseArg<Args, symbol!("target_chain_id"), Parsed = Chain::ChainId>
        + CanParseArg<Args, symbol!("target_client_id"), Parsed = Chain::ClientId>
        + CanParseArg<Args, symbol!("target_port_id"), Parsed = Chain::PortId>
        + CanParseArg<Args, symbol!("counterparty_chain_id"), Parsed = Counterparty::ChainId>
        + CanParseArg<Args, symbol!("counterparty_client_id"), Parsed = Counterparty::ClientId>
        + CanParseArg<Args, symbol!("counterparty_port_id"), Parsed = Counterparty::PortId>
        + CanParseArg<Args, symbol!("init_channel_options"), Parsed = Chain::InitChannelOptions>,
    App::Logger: CanLog<LevelInfo>,
    Args: Async,
    Builder: CanBuildRelay<Index<0>, Index<1>, Relay = Relay>
        + HasChainTypeAt<Index<0>, Chain = Chain>
        + HasChainTypeAt<Index<1>, Chain = Counterparty>
        + HasRelayTypeAt<Index<0>, Index<1>>,
    Chain: HasChainIdType
        + HasClientIdType<Counterparty>
        + HasInitChannelOptionsType<Counterparty>
        + HasIbcChainTypes<Counterparty>
        + HasErrorType,
    Chain::InitChannelOptions: Default,
    Chain::ChainId: Display,
    Chain::ClientId: Display,
    Chain::ChannelId: Display,
    Counterparty::ChainId: Display,
    Counterparty::ClientId: Display,
    Counterparty::ChannelId: Display,
    Counterparty: HasChainIdType + HasClientIdType<Chain> + HasIbcChainTypes<Chain> + HasErrorType,
    Relay: CanBootstrapChannel + HasRelayChains<SrcChain = Chain, DstChain = Counterparty>,
{
    async fn run_command(app: &App, args: &Args) -> Result<App::Output, App::Error> {
        let logger = app.logger();
        let builder = app.load_builder().await?;

        let target_chain_id = app.parse_arg(args, PhantomData::<symbol!("target_chain_id")>)?;
        let target_client_id = app.parse_arg(args, PhantomData::<symbol!("target_client_id")>)?;
        let target_port_id = app.parse_arg(args, PhantomData::<symbol!("target_port_id")>)?;
        let counterparty_chain_id =
            app.parse_arg(args, PhantomData::<symbol!("counterparty_chain_id")>)?;
        let counterparty_client_id =
            app.parse_arg(args, PhantomData::<symbol!("counterparty_client_id")>)?;
        let counterparty_port_id =
            app.parse_arg(args, PhantomData::<symbol!("counterparty_port_id")>)?;

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

        logger
            .log(
                &format!(
                    "Creating channel between {}:{} and {}:{} ...",
                    target_chain_id,
                    target_client_id,
                    counterparty_chain_id,
                    counterparty_client_id,
                ),
                &LevelInfo,
            )
            .await;

        let (target_channel_id, counterparty_channel_id) = relay
            .bootstrap_channel(&target_port_id, &counterparty_port_id, &Default::default())
            .await
            .map_err(App::raise_error)?;

        logger
            .log(
                &format!(
                    "Channel {}:{} successfully created between {} and {}",
                    target_channel_id,
                    counterparty_channel_id,
                    target_chain_id,
                    counterparty_chain_id,
                ),
                &LevelInfo,
            )
            .await;

        Ok(app.produce_output("Done"))
    }
}
