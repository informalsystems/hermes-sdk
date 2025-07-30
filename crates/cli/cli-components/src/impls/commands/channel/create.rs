use core::fmt::Display;
use core::marker::PhantomData;

use cgp::core::field::Index;
use hermes_core::logging_components::traits::CanLog;
use hermes_core::logging_components::types::LevelInfo;
use hermes_core::relayer_components::build::traits::builders::relay_builder::CanBuildRelay;
use hermes_core::relayer_components::chain::traits::{
    HasChainIdType, HasClientIdType, HasIbcChainTypes, HasInitChannelOptionsType,
};
use hermes_core::relayer_components::multi::traits::chain_at::HasChainTypeAt;
use hermes_core::relayer_components::multi::traits::relay_at::HasRelayTypeAt;
use hermes_core::relayer_components::relay::impls::CanBootstrapChannel;
use hermes_core::relayer_components::relay::traits::HasRelayChains;
use hermes_prelude::*;

use crate::traits::{
    CanLoadBuilder, CanParseArg, CanProduceOutput, CommandRunner, CommandRunnerComponent,
    HasOutputType,
};

#[cgp_new_provider(CommandRunnerComponent)]
impl<App, Args, Builder, Chain, Counterparty, Relay> CommandRunner<App, Args>
    for RunCreateChannelCommand
where
    App: CanLoadBuilder<Builder = Builder>
        + HasOutputType
        + HasErrorType
        + CanLog<LevelInfo>
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
    Args: Async,
    Builder: CanBuildRelay<Index<0>, Index<1>, Relay = Relay>
        + HasChainTypeAt<Index<0>, Chain = Chain>
        + HasChainTypeAt<Index<1>, Chain = Counterparty>
        + HasRelayTypeAt<Index<0>, Index<1>>,
    Chain: HasChainIdType
        + HasErrorType
        + HasClientIdType<Counterparty>
        + HasInitChannelOptionsType<Counterparty>
        + HasIbcChainTypes<Counterparty>,
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

        let init_channel_options =
            app.parse_arg(args, PhantomData::<symbol!("init_channel_options")>)?;

        let relay = builder
            .build_relay(
                PhantomData::<(Index<0>, Index<1>)>,
                &target_chain_id,
                &counterparty_chain_id,
                &target_client_id,
                &counterparty_client_id,
                None, // client refresh not needed for this CLI
                None, // client refresh not needed for this CLI
            )
            .await
            .map_err(App::raise_error)?;

        app.log(
            &format!(
                "Creating channel between {target_chain_id}:{target_client_id} \
                and {counterparty_chain_id}:{counterparty_client_id} ..."
            ),
            &LevelInfo,
        )
        .await;

        let (target_channel_id, counterparty_channel_id) = relay
            .bootstrap_channel(
                &target_port_id,
                &counterparty_port_id,
                &init_channel_options,
            )
            .await
            .map_err(App::raise_error)?;

        app.log(
            &format!(
                "Channel {target_channel_id}:{counterparty_channel_id} successfully \
                created between {target_chain_id} and {counterparty_chain_id}"
            ),
            &LevelInfo,
        )
        .await;

        Ok(app.produce_output("Done"))
    }
}
